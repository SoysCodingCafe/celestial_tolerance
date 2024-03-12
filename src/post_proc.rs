// Post processing module, for shader effects and chromatic aberration
use bevy::{
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{
            ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
        },
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
        RenderApp,
    },
};

use crate::{helper::GameState, setup::{BellEvent, ORTHO}};

pub struct PostProcPlugin;

impl Plugin for PostProcPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugins((
				ExtractComponentPlugin::<PostProcessSettings>::default(),
				UniformComponentPlugin::<PostProcessSettings>::default(),
			))
			.add_systems(Update, (
				update_settings,
			))
		;

		let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

		render_app
			.add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(
				Core2d,
				PostProcessLabel,
			)
			.add_render_graph_edges(
				Core2d,
				(
					Node2d::Tonemapping,
					PostProcessLabel,
					Node2d::EndMainPassPostProcessing,
				),
			)
		;
	}

	fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<PostProcessPipeline>()
		;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostProcessLabel;

#[derive(Default)]
struct PostProcessNode;

impl ViewNode for PostProcessNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static PostProcessSettings,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _post_process_settings): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<PostProcessPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let settings_uniforms = world.resource::<ComponentUniforms<PostProcessSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &post_process_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &post_process_pipeline.sampler,
                settings_binding.clone(),
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

#[derive(Resource)]
struct PostProcessPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<PostProcessSettings>(false),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world
            .resource::<AssetServer>()
            .load("shaders/post_processing.wgsl");

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("post_process_pipeline".into()),
                layout: vec![layout.clone()],
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PostProcessSettings {
	pub location: Vec2,
    pub time: f32,
	pub start_time: f32,
	// pub end_time: f32,
    // #[cfg(feature = "webgl2")]
    // _webgl2_padding: Vec3,
}

fn update_settings(
	mut settings: Query<&mut PostProcessSettings>,
	time: Res<Time>,
	current_state: Res<State<GameState>>,
	mut ev_r_bell: EventReader<BellEvent>,
) {
    for mut setting in &mut settings {
		setting.time = time.elapsed_seconds();
		//println!("{}", setting.radius);

		for ev in ev_r_bell.read() {
				setting.location = ev.location;
				setting.location.x = setting.location.x/ORTHO.x;
				setting.location.y = -setting.location.y/ORTHO.y;
				setting.start_time = time.elapsed_seconds()*100.0;
				//println!("Elapsed Time: {}", setting.start_time/100.0);
				// setting.end_time = match ev.selected_bell {
				// 	0 => 0.7,
				// 	1 => 0.6,
				// 	2 => 3.0,
				// 	3 => 1.0,
				// 	_ => 1.0,
				// };

				setting.start_time = setting.start_time + match ev.selected_bell {
					0 => (7<<16) as f32,
					1 => (6<<16) as f32,
					2 => (30<<16) as f32,
					3 => (10<<16) as f32,
					_ => (10<<16) as f32,
				};
				//println!("Big Time: {}", setting.start_time);
				//println!("Small Time: {}", (setting.start_time as u32 & 65535) as f32/100.0);
		}

		if *current_state != GameState::Game {
			setting.start_time = -100.0;
		}
    }
}