extends Control


@onready var param_box = %param_box

var deagable_texture_scene = preload("res://dragable_texture.tscn")


# @onready var size_box: ParamBox = ParamBox.create(param_box,"Size", 256, 1, 4000)
# @onready var iters_box: ParamBox = ParamBox.create(param_box,"Iters", 1, 1, 4000)
# @onready var sp_box: ParamBox = ParamBox.create(param_box,"SP", 1, 1, 4000)
@onready var drops_box: ParamBox = ParamBox.create(param_box,"drops", 1, 1, 400000)
@onready var life_box: ParamBox = ParamBox.create(param_box,"life", 1, 1, 400000)
@onready var erosion: ParamBox = ParamBox.create(param_box,"erosion", 0.01, 0, 100,0.01)

func _on_button_pressed() -> void:
	print_debug("A")
	var c = Caller.new()
	# var call: Callable = c.godot_basic_drop.bind(int(drops_box.value),int(life_box.value),erosion.value)
	# call.call()
	print_debug("H")
	c.godot_bfd_step(2)
	print_debug("H2")
	# print_debug(int(size_box.value))
	# c.generate_image_from_array(heightmap)
	


	var img = Image.load_from_file("TEST.png")
	var texture = ImageTexture.create_from_image(img)
	var dragable_texture: DragableTexture = deagable_texture_scene.instantiate()

	

	add_child(dragable_texture)
	dragable_texture.set_texture(texture)
	dragable_texture.set_description(str(call.get_bound_arguments()))
	dragable_texture.global_position = get_viewport_rect().size / 2
