extends Node2D


@onready var param_box = %param_box

var deagable_texture_scene = preload("res://dragable_texture.tscn")

func _ready() -> void:
	pass

func _on_button_pressed() -> void:
	var c = Caller.new()
	var heightmap = c.perlin(256,256,1.1,2);
	c.generate_image_from_array(heightmap)
	


	var img = Image.load_from_file("TEST.png")
	

	var texture = ImageTexture.create_from_image(img)

	var dragable_texture = deagable_texture_scene.instantiate()
	dragable_texture.texture = texture

	add_child(dragable_texture)

	pass # Replace with function body.
