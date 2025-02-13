import ctypes  # needed to interact with python at a lower level
import pyglet
import math
import os

pyglet.options["shadow_window"] = False  # no need for shadow window
pyglet.options["debug_gl"] = False  # makes things slow, so disable it

import pyglet.gl as gl
import shader
import matrix
import block_type
import texture_manager

from utils import base_dir


class Window(pyglet.window.Window):  # create a class extending pyglet.window.Window
    def __init__(self, **args):
        super().__init__(**args)  # pass on arguments to pyglet.window.Window.__init__ function

        # create blocks
        self.texture_manager = texture_manager.TextureManager(
            16, 16, 256
        )  # create our texture manager (256 textures that are 16 * 16 pixel each)

        self.cobblestone = block_type.BlockType(
			self.texture_manager, "cobblestone", {"all": "cobblestone"}
		)  # create each one of our blocks with the texture manager and a list of textures per face
        self.grass = block_type.BlockType(
            self.texture_manager, "grass", {"top": "grass", "bottom": "dirt", "sides": "grass_side"}
		)
        self.dirt = block_type.BlockType(self.texture_manager, "dirt", {"all": "dirt"})
        self.stone = block_type.BlockType(self.texture_manager, "stone", {"all": "stone"})
        self.sand = block_type.BlockType(self.texture_manager, "sand", {"all": "sand"})
        self.planks = block_type.BlockType(self.texture_manager, "planks", {"all": "planks"})
        self.log = block_type.BlockType(
			self.texture_manager, "log", {"top": "log_top", "bottom": "log_top", "sides": "log_side"}
		)

        self.texture_manager.generate_mipmaps()  # generate mipmaps for our texture manager's texture

        # create vertex array object
        self.vao = gl.GLuint(0)
        gl.glGenVertexArrays(1, ctypes.byref(self.vao))
        gl.glBindVertexArray(self.vao)

        # create vertex position vbo
        self.vertex_position_vbo = gl.GLuint(0)
        gl.glGenBuffers(1, ctypes.byref(self.vertex_position_vbo))
        gl.glBindBuffer(gl.GL_ARRAY_BUFFER, self.vertex_position_vbo)

        gl.glBufferData(
			gl.GL_ARRAY_BUFFER,
			ctypes.sizeof(gl.GLfloat * len(self.grass.vertex_positions)),
			(gl.GLfloat * len(self.grass.vertex_positions))(
				*self.grass.vertex_positions
			),  # use grass block's vertex positions
			gl.GL_STATIC_DRAW,
		)

        gl.glVertexAttribPointer(0, 3, gl.GL_FLOAT, gl.GL_FALSE, 0, 0)
        gl.glEnableVertexAttribArray(0)

		# create tex coord vbo
        self.tex_coord_vbo = gl.GLuint(0)
        gl.glGenBuffers(1, ctypes.byref(self.tex_coord_vbo))
        gl.glBindBuffer(gl.GL_ARRAY_BUFFER, self.tex_coord_vbo)

        gl.glBufferData(
			gl.GL_ARRAY_BUFFER,
			ctypes.sizeof(gl.GLfloat * len(self.grass.tex_coords)),
			(gl.GLfloat * len(self.grass.tex_coords))(
				*self.grass.tex_coords
			),  # use grass block's texture coordinates positions
			gl.GL_STATIC_DRAW,
		)

        gl.glVertexAttribPointer(1, 3, gl.GL_FLOAT, gl.GL_FALSE, 0, 0)
        gl.glEnableVertexAttribArray(1)

		# create index buffer object
        self.ibo = gl.GLuint(0)
        gl.glGenBuffers(1, self.ibo)
        gl.glBindBuffer(gl.GL_ELEMENT_ARRAY_BUFFER, self.ibo)

        gl.glBufferData(
			gl.GL_ELEMENT_ARRAY_BUFFER,
			ctypes.sizeof(gl.GLuint * len(self.grass.indices)),
			(gl.GLuint * len(self.grass.indices))(*self.grass.indices),  # use grass block's indices
			gl.GL_STATIC_DRAW,
		)

        # create shader
        self.shader = shader.Shader(
            os.path.join(base_dir, "shaders", "vert.glsl"),
            os.path.join(base_dir, "shaders", "frag.glsl")
        )
        self.shader_matrix_location = self.shader.find_uniform(b"matrix")  # get the shader matrix uniform location
        self.shader_sampler_location = self.shader.find_uniform(b"texture_array_sampler")  # find our texture array sampler's uniform
        self.shader.use()

        # create matrices
        self.mv_matrix = matrix.Matrix()  # modelView
        self.p_matrix = matrix.Matrix()  # projection

        self.x = 0  # temporary variable
        pyglet.clock.schedule_interval(self.update, 1.0 / 60)  # call update function every 60th of a second

    def update(self, delta_time):
        self.x += delta_time  # increment self.x consistently

    def on_draw(self):
        # create projection matrix
        self.p_matrix.load_identity()
        self.p_matrix.perspective(90, float(self.width) / self.height, 0.1, 500)

        # create model view matrix
        self.mv_matrix.load_identity()
        self.mv_matrix.translate(0, 0, -3)
        self.mv_matrix.rotate_2d(self.x, math.sin(self.x / 3 * 2) / 2)

        # modelviewprojection matrix
        mvp_matrix = self.p_matrix * self.mv_matrix
        self.shader.uniform_matrix(self.shader_matrix_location, mvp_matrix)

        # bind textures
        gl.glActiveTexture(gl.GL_TEXTURE0)  # set our active texture unit to the first texture unit
        gl.glBindTexture(
            gl.GL_TEXTURE_2D_ARRAY, self.texture_manager.texture_array
        )  # bind out texture manager's texture
        gl.glUniform1i(
            self.shader_sampler_location, 0
        )  # tell our sampler our texture is bound to the first texture unit

        # draw stuff
        gl.glEnable(gl.GL_DEPTH_TEST)
        gl.glClearColor(0.0, 0.0, 0.0, 1.0)  # set clear color
        self.clear()  # clear screen

        gl.glDrawElements(gl.GL_TRIANGLES, len(self.grass.indices), gl.GL_UNSIGNED_INT, None)  # draw bound buffers to the screen

    def on_resize(self, width: int, height: int):
        print(f"Resize {width} * {height}")  # print out window size
        gl.glViewport(0, 0, width, height)


class Game:
    def __init__(self):
        self.config = gl.Config(double_buffer=True, major_version=3, minor_version=3, depth_size=16)  # use modern opengl
        self.window = Window(
            config=self.config, caption="Voxel Engine",
            width=800, height=600, resizable=True,
            vsync=False  # vsync with pyglet causes problems on some computers, so disable it
        )

    def run(self):
        pyglet.app.run()  # run our application


if __name__ == "__main__":  # only run the game if source file is the one run
    game = Game()  # create game object
    game.run()
