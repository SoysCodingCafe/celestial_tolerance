// This shader computes the chromatic aberration effect
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct PostProcessSettings {
	location: vec2<f32>,
    time: f32,
	start_time: f32,
// 	end_time: f32,
// #ifdef SIXTEEN_BYTE_ALIGNMENT
//     // WebGL2 structs must be 16 byte aligned.
//     _webgl2_padding: vec3<f32>
// #endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(
	in: FullscreenVertexOutput,
) -> @location(0) vec4<f32> {
	// Chromatic aberration strength
    var offset_strength = 0.0;

	var uv = in.uv;
	var temp = vec4<f32>(0.0, 0.0, 0.0, 0.0);
	temp = bell_ripple_2(uv);
	uv = temp.xy;

	var color = textureSample(screen_texture, texture_sampler, uv);

	// Sample each color channel with an arbitrary shift
	offset_strength = temp.z;
	let r_offset = cart_to_iso(vec2<f32>(offset_strength * uv.x, 0.0))*0.6;
	let g_offset = cart_to_iso(vec2<f32>(-offset_strength * uv.x, 0.0))*0.6;
	let b_offset = cart_to_iso(vec2<f32>(0.0, -offset_strength * uv.y))*0.6;
	color = vec4<f32>(
        textureSample(screen_texture, texture_sampler, in.uv + r_offset).r,
        textureSample(screen_texture, texture_sampler, in.uv + g_offset).g,
        textureSample(screen_texture, texture_sampler, in.uv + b_offset).b,
        1.0
    );

	color = color + (temp.z * temp.w);

	return color;
	//return vec4<f32>(uv.x, uv.y, 0.0, 1.0);
}

fn bell_ripple_2(
	uv: vec2<f32>,
) -> vec4<f32> {
	var uv_out = uv;
	var boost = 0.0;
	var fade = 1.0;

	var start_time = f32(u32(settings.start_time) & u32(65535))/100.0;
	var end_time = (settings.start_time / pow(2.0, 16.0))/10.0;

	var t = settings.time;
	let st = start_time;
	let dt = t-st;
	let et = st + end_time;
	let tt = et-st;
	let f = dt/tt;
	let l = settings.location;
	let iv = 2.0; // Inverse velocity

	var rel_uv = uv;
	rel_uv = uv - l;
	rel_uv = (rel_uv * 2.0) - 1.0;
	rel_uv.x = rel_uv.x / 9.0;
	rel_uv.y = rel_uv.y / 8.0;

	let cl = length(rel_uv);
	if dt >= 0.0 && dt < tt {
		if cl < 0.1 * dt/iv 
		&& cl > 0.09 * dt/iv {
			boost = 0.2;
			fade = pow(1.0-f, 2.0);
		}
	}

	//uv_out = rel_uv;
	//boost = 1.0-f;
	// else if dt < et + 0.5 {
	// 	if cl < 0.1 * dt/iv 
	// 	&& cl > 0.09 * dt/iv {
	// 		boost = 1.0;
	// 		fade = (et+0.5-dt);
	// 	}
	// }

	return vec4<f32>(rel_uv, boost, fade);
}

fn cart_to_iso(
	in: vec2<f32>,
) -> vec2<f32> {
	//let abcd = vec4<f32>(1.0, -1.0, -0.5, -0.5);
	let abcd = vec4<f32>(1.0, 1.0, -0.5, 0.5);
	return vec2<f32>(in.x*abcd.x+in.y*abcd.y, in.x*abcd.z+in.y*abcd.w);
}

// fn bell_ripple(
// 	uv: vec2<f32>,
// ) -> vec3<f32> {
// 	var uv_out = uv;
// 	var boost = 1.0;

// 	var t = settings.time;
// 	let st = settings.start_time;
// 	let l = settings.location;
// 	if t >= st && t < st + settings.end_time {
// 		var rel_uv = uv;
// 		rel_uv = uv - l; 
// 		rel_uv = (rel_uv * 2.0) - 1.0;
// 		//rel_uv.x = rel_uv.x * 16.0/9.0;

// 		rel_uv = cart_to_iso(rel_uv)/5.0;
// 		let cl = length(rel_uv);
// 		//uv_out = uv + (rel_uv/cl) * cos(cl * 10.0 - (t - st) * 4.0) * 0.02;
		
// 		if cl > (sin((t - 4.75 - st)%3.14) + 1.0)/2.0 
// 		&& cl < (sin((t - 4.75 - st)%3.14+cl/2.0) + 1.0)/2.0 {
// 			//uv_out = uv - normalize(rel_uv) * 0.1 * exp(-(cl*cl)/(0.1*0.1));
// 			uv_out = uv + (rel_uv/cl) * cos(cl * 10.0 - (t - st) * 4.0) * 0.02;
// 			// uv_out = vec2<f32>(0.0, 0.0);
// 			boost = 2.0;
// 		}
// 		return vec3<f32>(uv_out, boost);
// 	} else {
// 		return vec3<f32>(uv_out, boost);
// 	}
// }