const vsSource = `
attribute vec2 a_pos;
varying vec2 v_uv;

void main() {
  v_uv = a_pos * 0.5 + 0.5;
  gl_Position = vec4(a_pos, 0.0, 1.0);
}
`;

const fsSource = `
precision highp float;

varying vec2 v_uv;
uniform float u_time;
uniform vec2 u_res;

float hash2(vec2 p) {
  p = fract(p * vec2(443.897, 441.423));
  p += dot(p, p.yx + 19.19);
  return fract((p.x + p.y) * p.x);
}

float rand(float n){return fract(sin(n) * 43758.5453123);}

float noise(float p){
    float fl = floor(p);
    float fc = fract(p);
    fc = smoothstep(0.0, 1.0, fc);
    return mix(rand(fl), rand(fl + 1.0), fc);
}

const vec2 LANTERN_POS = vec2(0.06, 0.25);
const vec3 LANTERN_COLOR = vec3(0.98, 0.60, 0.15);

float lantern_influence(vec2 uv) {
  vec2 diff = uv - LANTERN_POS;
  float dist = length(diff);

  // Warm glow falloff — strong near, fading far
  float inner = exp(-dist * 8.0);
  float outer = exp(-dist * 1.2);
  return inner * 0.8 + outer * 0.35;
}

vec3 background(vec2 uv) {
  // colors
  vec3 sky = vec3(0.10, 0.10, 0.155);
  vec3 ground = vec3(0.094, 0.094, 0.141);

  // gradient
  vec3 col = ground;
  col = mix(col, sky, smoothstep(0.4, 1.0, uv.y));

  // lantern glow on background
  vec2 diff = uv - LANTERN_POS;
  float lantern_dist = length(diff * vec2(1.0, 1.6));
  col += LANTERN_COLOR * 0.55 * exp(-lantern_dist * 15.0);
  col += LANTERN_COLOR * 0.35 * exp(-lantern_dist * 3.5);

  return col;
}

float splash(vec2 uv, float grid_size, float seed, float rate) {
  float intensity = 0.0;

  vec2 scaled = uv * grid_size;
  vec2 cell = floor(scaled);
  vec2 local = fract(scaled);

  for (int dx = -1; dx <= 1; dx++) {
    {
      int dy = 0;
      vec2 neighbor = cell + vec2(float(dx), float(dy));
      float cell_h = hash2(neighbor + seed);

      if (cell_h > 0.6) continue;

      float cycle_len = 0.4 + 0.6 * hash2(neighbor * 3.1 + seed + 30.0);
      float time_offset = hash2(neighbor * 4.9 + seed + 40.0) * cycle_len;
      float t = mod(u_time * rate + time_offset, cycle_len);
      float life_phase = t / cycle_len;

      if (life_phase > 0.7) continue;

      vec2 splash_pos = vec2(
        hash2(neighbor * 1.3 + seed + 10.0),
        hash2(neighbor * 2.7 + seed + 20.0)
      );
      float normalized_life = life_phase / 0.7;

      vec2 delta = (local - splash_pos + vec2(float(dx), float(dy)));
      float dist = length(delta);

      float max_radius = 0.25 + 0.15 * hash2(neighbor * 5.3 + seed + 50.0);
      float ring_radius = normalized_life * max_radius;
      float ring_width = 0.04 + 0.03 * (1.0 - normalized_life);

      float ring = 1.0 - smoothstep(ring_width * 0.3, ring_width, abs(dist - ring_radius));

      float fade = 1.0 - normalized_life;
      fade = fade * fade;

      float center_dot = exp(-dist * dist * 800.0) * max(0.0, 1.0 - normalized_life * 3.0);

      float brightness = 0.5 + 0.5 * hash2(neighbor * 6.7 + seed + 60.0);

      intensity += (ring * fade + center_dot) * brightness;
    }
  }

  return clamp(intensity, 0.0, 1.0);
}

float ground_splash(vec2 uv, float grid_size, float seed, float rate) {
  float perspective_squash = 2.5 + (1.0 - uv.y) * 2.0;
  vec2 ground_uv = vec2(uv.x, uv.y * perspective_squash);
  return splash(ground_uv, grid_size, seed, rate);
}

float rain_layer(vec2 uv, float density, float speed, float thickness, float length, float angle, float seed) {
  float intensity = 0.0;

  float s = sin(angle);
  float c = cos(angle);
  uv = vec2(c * uv.x - s * uv.y, s * uv.x + c * uv.y);

  vec2 scaled = uv * vec2(density, density * 0.15);
  scaled.y += u_time * speed;

  vec2 cell = floor(scaled);
  vec2 local = fract(scaled);

  for (int dx = -1; dx <= 1; dx++) {
    vec2 neighbor = cell + vec2(float(dx), 0.0);
    float cell_hash = hash2(neighbor + seed);

    if (cell_hash < 0.35) continue;

    float streak_x = hash2(neighbor * 1.7 + seed + 0.5) * 0.8 + 0.1;
    float streak_brightness = 0.4 + 0.6 * hash2(neighbor * 2.3 + seed + 1.0);
    float streak_length = (0.3 + 0.7 * hash2(neighbor * 3.1 + seed + 2.0)) * length;

    float streak_y_start = hash2(neighbor * 4.7 + seed + 3.0);

    float dy = local.y - streak_y_start;

    if (dy < 0.0) dy += 1.0;

    float y_mask = 1.0 - smoothstep(0.0, streak_length, dy);
    y_mask *= smoothstep(0.0, 0.05, dy);

    float local_x = local.x - streak_x + float(dx);
    float x_dist = abs(local_x);

    float half_width = thickness * 0.5;
    float x_mask = 1.0 - smoothstep(half_width * 0.3, half_width, x_dist);

    intensity += x_mask * y_mask * streak_brightness;
  }

  return clamp(intensity, 0.0, 1.0);
}

void main() {
  vec2 uv = v_uv;
  float aspect = u_res.x / u_res.y;
  uv = vec2(uv.x * aspect, uv.y); // aspectified

  vec3 col = background(uv);

  float lantern_inf = lantern_influence(uv);

  float ground_zone = 1.0 - smoothstep(0.05, 0.15, uv.y);

  if (ground_zone > 0.001) {
    float s1 = ground_splash(uv, 18.0, 0.0, 2.3);
    float s2 = ground_splash(uv, 14.0, 500.0, 3.2);

    float splash_total = s1 * 0.5 + s2 * 0.4;
    splash_total *= ground_zone;

    // Blend splash color + lantern
    vec3 splash_base = vec3(0.75, 0.80, 0.85);
    vec3 splash_warm = LANTERN_COLOR * 1.2;
    vec3 splash_color = mix(splash_base, splash_warm, lantern_inf * 4.0);

    // Boost splash brightness very dependent on lantern
    float splash_bright = 0.001 + lantern_inf * 1.7;
    col += splash_color * splash_total * splash_bright;
  }

  // Fade rain out near ground
  float rain_fade_2 = smoothstep(0.0, 0.04, uv.y);
  float rain_fade_3 = smoothstep(0.0, 0.12, uv.y);

  // Rain layer colors
  vec3 rain_cool_2 = vec3(0.45, 0.50, 0.55);
  vec3 rain_cool_3 = vec3(0.80, 0.85, 0.90);
  vec3 rain_warm = LANTERN_COLOR * 1.3;

  float r2 = rain_layer(uv, 60.0, 8.5, 0.15, 0.7, 0.09, 200.0);
  vec3 r2_color = mix(rain_cool_2, rain_warm, lantern_inf * 2.0);
  float r2_bright = 0.08 + lantern_inf * 0.40;
  col += r2_color * r2 * r2_bright * rain_fade_2;

  float r3 = rain_layer(uv, 120.0, 9.5, 0.08, 0.4, 0.15, 700.0);
  vec3 r3_color = mix(rain_cool_3, rain_warm, lantern_inf * 2.0);
  float r3_bright = 0.18 + lantern_inf * 0.55;
  col += r3_color * r3 * r3_bright * rain_fade_3;

  gl_FragColor = vec4(col, 1.0);
}
`;

const canvas = document.getElementById("rain-canvas");
const gl = canvas.getContext('webgl', { antialias: false });

function resize() {
  const dpr = window.devicePixelRatio || 1;
  canvas.width = window.innerWidth * dpr;
  canvas.height = window.innerHeight * dpr;
  gl.viewport(0, 0, canvas.width, canvas.height);
}
resize();
window.addEventListener('resize', resize);

function createShader(source, type) {
  const shader = gl.createShader(type);

  gl.shaderSource(shader, source);
  gl.compileShader(shader);

  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    console.error("Couldn't compile shader", gl.getShaderInfoLog(shader));
    gl.deleteShader(shader);
    return null;
  }
  return shader;
}

function createProgram(shaders) {
  const program = gl.createProgram();
  for (const shader of shaders) {
    gl.attachShader(program, shader);
  }

  gl.linkProgram(program);

  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    console.error("Couldn't link program", gl.getProgramInfoLog(program));
    gl.deleteProgram(program);
    return null;
  }
  return program;
}

const vShader = createShader(vsSource, gl.VERTEX_SHADER);
const fShader = createShader(fsSource, gl.FRAGMENT_SHADER);

const program = createProgram([vShader, fShader]);

if (!program) {
  throw new Error('no program?');
}

gl.useProgram(program);

const buf = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, buf);
gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);

const aPos = gl.getAttribLocation(program, 'a_pos');
gl.enableVertexAttribArray(aPos);
gl.vertexAttribPointer(aPos, 2, gl.FLOAT, false, 0, 0);

const uTime = gl.getUniformLocation(program, 'u_time');
const uRes = gl.getUniformLocation(program, 'u_res');

const timeOffset = Math.random() * 10;

function frame(t) {
  gl.uniform1f(uTime, (t * 0.001 + timeOffset) % 1000)
  gl.uniform2f(uRes, canvas.width, canvas.height);
  gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
  requestAnimationFrame(frame);
}
requestAnimationFrame(frame);

