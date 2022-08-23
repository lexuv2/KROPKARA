extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var col = Color(255,255,255)

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
	#pass



func _on_point_draw():
	draw_rect(Rect2( float(position.x),float(position.y),2,2) , col,true,1);
	pass # Replace with function body.
