mod accelerators;
mod cameras;
mod core;
mod integrators;
mod materials;
mod samplers;
mod shapes;
mod test_cases;
mod tools;

use crate::core::pbrt::*;
use crate::core::renderer::*;
use crate::test_cases::configurations::*;

fn bvh_test(ratio: f32) {
    let width = (1920 as f32 * ratio) as usize;
    let height = (1080 as f32 * ratio) as usize;

    render(
        create_dragon_in_the_air(),
        1,
        width,
        height,
        &format!("bvh_dragon"),
    );

    render(
        create_transformed_dragon_in_the_air(),
        1,
        width,
        height,
        &format!("bvh_dragon_transformed"),
    );

    render(
        create_bvh_many_dragons(),
        1,
        width,
        height,
        &format!("many_dragons"),
    );
}

fn rt_weekend(num_samples: u32, ratio: f32) {
    let width = (1920 as f32 * ratio) as usize;
    let height = (1080 as f32 * ratio) as usize;

    let config_rt_weekend = create_rt_weekend();
    render(
        config_rt_weekend
            .update_integrator(Arc::new(DebuggerScatterRay::default()))
            .update_camera(
                config_rt_weekend
                    .camera
                    .reset_lens_and_focus_distance(0.0, f32::NAN),
            ),
        4,
        width,
        height,
        &format!("rt_weekend_scatter_ray"),
    );

    render(
        config_rt_weekend,
        num_samples,
        width,
        height,
        &format!("rt_weekend"),
    );
}

fn cornel_box(num_samples: u32, ratio: f32) {
    let width = (1080 as f32 * ratio) as usize;
    let height = (1080 as f32 * ratio) as usize;

    let integrator_nee = Arc::new(NextEventEstimation::default());

    let cornell_box_lambertian = create_cornell_box_lambertian();
    render(
        cornell_box_lambertian.clone(),
        num_samples,
        width,
        height,
        &format!("cornell_box_lambertian_pt_{}", num_samples),
    );

    render(
        cornell_box_lambertian.update_integrator(integrator_nee.clone()),
        num_samples,
        width,
        height,
        &format!("cornell_box_lambertian_nee_{}", num_samples),
    );

    let cornell_box_specular = create_cornell_box_specular();
    render(
        cornell_box_specular.clone(),
        num_samples,
        width,
        height,
        &format!("cornell_box_specular_pt_{}", num_samples),
    );

    render(
        cornell_box_specular.update_integrator(integrator_nee.clone()),
        num_samples,
        width,
        height,
        &format!("cornell_box_specular_nee_{}", num_samples),
    );
}

fn cornel_box_dragon(num_samples: u32, ratio: f32) {
    let width = (1080 as f32 * ratio) as usize;
    let height = (1080 as f32 * ratio) as usize;

    let integrator_nee = Arc::new(NextEventEstimation::default());
    let integrator_normal = Arc::new(DebuggerIntersectNormal::default());

    let cornell_box_dragon = create_cornell_box_dragon();
    render(
        cornell_box_dragon.update_integrator(integrator_normal.clone()),
        4,
        width,
        height,
        &format!("cornell_box_dragon_normal"),
    );

    render(
        cornell_box_dragon.update_integrator(integrator_nee.clone()),
        num_samples,
        width,
        height,
        &format!("cornell_box_dragon_nee_{}", num_samples),
    );
}

fn smallpt(num_samples: u32, ratio: f32) {
    let width = (2048 as f32 * ratio) as usize;
    let height = (1524 as f32 * ratio) as usize;
    let smallpt = create_smallpt();
    let integrator_nee = Arc::new(NextEventEstimation::default());

    render(
        smallpt.clone(),
        num_samples,
        width,
        height,
        &format!("smallpt_pt_{}", num_samples),
    );
    render(
        smallpt.update_integrator(integrator_nee.clone()),
        num_samples,
        width,
        height,
        &format!("smallpt_nee_{}", num_samples),
    );
}

fn main() {
    let num_samples = 16;
    let ratio = 1.0;

    println!();
    bvh_test(ratio);
    rt_weekend(num_samples, ratio);
    cornel_box(num_samples, ratio);
    cornel_box_dragon(num_samples, ratio);
    smallpt(num_samples, ratio);
}
