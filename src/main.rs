#![feature(unboxed_closures, fn_traits)]
#![feature(const_trait_impl)]

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

fn test(num_samples: u32, ratio: f32) {
    println!();

    let width = (1920 as f32 * ratio) as usize;
    let height = (1080 as f32 * ratio) as usize;

    render(
        create_dragon_in_the_air(width, height),
        1,
        width,
        height,
        &format!("bvh_dragon"),
    );

    render(
        create_transformed_dragon_in_the_air(width, height),
        1,
        width,
        height,
        &format!("bvh_dragon_transformed"),
    );

    render(
        create_bvh_many_dragons(width, height),
        1,
        width,
        height,
        &format!("many_dragons"),
    );

    let config_rt_weekend = create_rt_weekend(width, height);
    render(
        config_rt_weekend
            .update_integrator(Arc::new(DebuggerScatterRay::default()))
            .update_camera(config_rt_weekend.camera.remove_lens()),
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

    let width = (1080 as f32 * ratio) as usize;
    let height = (1080 as f32 * ratio) as usize;

    let integrator_nee = Arc::new(NextEventEstimation::default());
    let integrator_normal = Arc::new(DebuggerIntersectNormal::default());

    let cornell_box_lambertian = create_cornell_box_lambertian(width, height);
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

    let cornell_box_specular = create_cornell_box_specular(width, height);
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

    let cornell_box_dragon = create_cornell_box_dragon(width, height);
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

    {
        let width = (2048 as f32 * ratio) as usize;
        let height = (1524 as f32 * ratio) as usize;
        let smallpt = create_smallpt(width, height);
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
}

fn main() {
    test(16, 1.0);
}
