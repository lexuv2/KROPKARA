extends Node2D


@onready var param_box = %param_box

var deagable_texture_scene = preload("res://dragable_texture.tscn")


# @onready var size_box: ParamBox = ParamBox.create(param_box,"Size", 256, 1, 4000)
# @onready var iters_box: ParamBox = ParamBox.create(param_box,"Iters", 1, 1, 4000)
# @onready var sp_box: ParamBox = ParamBox.create(param_box,"SP", 1, 1, 4000)
@onready var drops_box: ParamBox = ParamBox.create(param_box,"drops", 1, 1, 400000)
@onready var life_box: ParamBox = ParamBox.create(param_box,"life", 1, 1, 400000)
@onready var erosion: ParamBox = ParamBox.create(param_box,"erosion", 0.01, 0, 100,0.01)

func _on_button_pressed() -> void:
	var c = Caller.new()
	c.godot_basic_drop(int(drops_box.value),int(life_box.value),erosion.value)
	# print_debug(int(size_box.value))
	# c.generate_image_from_array(heightmap)
	


	var img = Image.load_from_file("TEST.png")
	

	var texture = ImageTexture.create_from_image(img)

	var dragable_texture = deagable_texture_scene.instantiate()
	dragable_texture.texture = texture

	add_child(dragable_texture)
	dragable_texture.global_position = get_viewport_rect().size / 2

	pass # Replace with function body.
