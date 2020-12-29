import { WebGlUtil } from "./web_gl";
import vsGLSL from "./shaders/screen_vertex_shader.glsl";
import fsGLSL from "./shaders/screen_fragment_shader.glsl";

export const WIDTH = 160;
export const HEIGHT = 144;

export class Screen {

    constructor(canvas) {
        this.canvas = canvas;
        this.gl = canvas.getContext("webgl");
        if (this.gl == null) {
            throw new Error("WebGL not working!");
        }

        let gl = this.gl;
        let program = WebGlUtil.createProgram(gl, vsGLSL, fsGLSL);
        this.program = program;

        // clip space coords
        this.vertexCoordinates = new Float32Array([
            -1, 1,
            1, 1,
            1, -1,
            -1, 1,
            1, -1,
            -1, -1,
        ]);

        // uv coords
        this.textureCoordinates = new Float32Array([
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
        ]);
        WebGlUtil.createAttribLocation(
            gl,
            program,
            this.vertexCoordinates,
            "a_position",
            gl.STATIC_DRAW,
        );
        WebGlUtil.createAttribLocation(
            gl,
            program,
            this.textureCoordinates,
            "a_texCoord",
            gl.STATIC_DRAW,
        );
        this.frame = WebGlUtil.createTexture(gl, gl.TEXTURE0);

        let textureId = gl.getUniformLocation(program, `u_image`);
        gl.uniform1i(textureId, 0);

        this.uniforms = {
            mosaic:  gl.getUniformLocation(program, `u_mosaic`),
            brightness: gl.getUniformLocation(program, `u_brightness`)
        }

        WebGlUtil.clear(gl, canvas);
    }

    createBuffer() {
        const length = WIDTH * HEIGHT * 4;
        if (this.buffer) {
            return this.buffer;
        } else {
            this.buffer = {
                data: new Uint8ClampedArray(new ArrayBuffer(length)),
                width: WIDTH,
                height: HEIGHT,
            };
            return this.buffer;
        }
    }

    render(buffer) {
        let gl = this.gl;

        gl.uniform1f(this.uniforms.brightness, 1);
        gl.uniform1f(this.uniforms.mosaic, 0);
        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);

        WebGlUtil.updateTexture(gl, this.frame, buffer);

        const itemSize = 2;
        const numItems = this.vertexCoordinates.length;

        let offset = 0;
        let count = numItems / itemSize;
        gl.drawArrays(gl.TRIANGLES, offset, count);
    }

}