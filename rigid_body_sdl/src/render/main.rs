use sdl_mesh_renderer::{
    CameraMode,
    CameraMover,
    Camera,
    Color,
    FPSManager,
    Input,
    math::{
	geometry::FiniteLine3d,
	matrix::Matrix3x3,
	rotation_matrix,
	vector::Vector3d,
    },
    obj_loader,
    polyhedron_meshes,
    Mesh,
    Renderer,
    StrResult,
    Transform,
};
use std::{
    env,
    f64::consts::PI,
};

fn main() -> StrResult<()> {
    let context = sdl2::init()?;
    let mut input = Input::new(&context)?;
    let mut renderer = Renderer::new("mesh", &context, (800, 600))?;
    let mut camera = Camera::default();
    let camera_mover = CameraMover {
	center: Vector3d::new(5., 5., 5.),
	camera_range: (0., 500.),
	theta_scale: 0.0045,
	wheel_scale: 1.,
	move_fact: 0.1,
	mode: CameraMode::Fps,
    };
    let mut fps_manager = FPSManager::new(60);
    let mut meshes = get_meshes()?;

    let mut theta = (0., 0.);

    let line = FiniteLine3d {
	start: Vector3d::new(-10., 0., 10.),
	end: Vector3d::new(10., 0., 10.),
    };
    
    while input.get() {
	camera_mover.move_camera(&input, &mut camera);
	renderer.clear((0, 0, 0));
	for (mesh, pos, color) in &meshes {
	    renderer.draw_mesh(&mesh, &pos, &camera, *color);
	}
	renderer.draw_line(&line, &camera, (255, 0, 0));
	
	mutate_transform(&mut meshes[0].1, &mut theta);
	
	renderer.present()?;
	fps_manager.sleep_to_next_frame();
    }
    
    Ok(())
}

fn get_meshes() -> StrResult<Vec<(Mesh, Transform, Color)>> {
    let mut ret = Vec::new();
    ret.push((
	polyhedron_meshes::aligned_cuboid(
	    &Vector3d::new(5., 5., 5.)
	),
	(Vector3d::new(0., 0., 10.), Matrix3x3::identity()),
	(0, 255, 0),
    ));
    /*
    ret.push((
	polyhedron_meshes::regular_icosahedron(3.),
	(Vector3d::new(0., 0., 10.), Matrix3x3::identity()),
	(0, 255, 0),
    ));
    */
    /*
    ret.push((
	polyhedron_meshes::icosphere(3., 5),
	(Vector3d::new(0., 0., 10.), Matrix3x3::identity()),
	(0, 255, 0),
    ));
    */
    let args = env::args();
    for arg in args {
	if arg.contains(".obj") {
	    ret.push((
		obj_loader::simple(arg).map_err(|e| e.to_string())?,
		(Vector3d::default(), Matrix3x3::identity()),
		(255, 0, 0),
	    ));
	    break;
	}
    }
    Ok(ret)
}

fn mutate_transform(transform: &mut Transform, theta: &mut (f64, f64)) {
    //transform.0[2] += 0.1;
    transform.1 = rotation_matrix::x(theta.0).mult(&rotation_matrix::y(theta.1));
    theta.0 += 0.05;
    theta.1 += 0.05;
    let pi2 = PI*2.;
    if theta.0 > pi2 {theta.0 -= pi2;}
    if theta.0 < pi2 {theta.0 += pi2;}
    if theta.1 > pi2 {theta.1 -= pi2;}
    if theta.1 < pi2 {theta.1 += pi2;}
}
