#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};
use macros::MIN_WIDE_BENCH_SIZE;
use mathbench::BenchValue;
use std::ops::Mul;

// returns self to check overhead of benchmark
fn bench_symmetric_matrix2_ret_self(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar symmetric matrix2 return self");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_unop!(b, op => ret_self, ty => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix2;
        bench_unop!(b, op => ret_self, ty => Matrix2<f32>)
    });
    bench_bevy_math_extensions!(group, |b| {
        use bevy_math_extensions::SymmetricMat2;
        bench_unop!(b, op => ret_self, ty => SymmetricMat2)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat2;
        bench_unop!(b, op => ret_self, ty => Mat2)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_unop!(b, op => ret_self, ty => Matrix2<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat2;
        bench_unop!(b, op => ret_self, ty => Mat2<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix2x2::M22;
        bench_unop!(b, op => ret_self, ty => M22<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Matrix2x2F;
        bench_unop!(b, op => ret_self, ty => Matrix2x2F)
    });
    group.finish();
}

fn bench_symmetric_matrix2_ret_self_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide symmetric matrix2 return self");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Mat2;
        bench_unop_wide!(b, size, width => 1, op => ret_self, ty => Mat2)
    });
    bench_bevy_math_extensions_f32x4!(group, size, |b, size| {
        use bevy_math_extensions::SymmetricMat2x4;
        bench_unop_wide!(b, size, width => 4, op => ret_self, ty => SymmetricMat2x4)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Mat2x4;
        bench_unop_wide!(b, size, width => 4, op => ret_self, ty => Mat2x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::Matrix2;
        use simba::simd::f32x4;
        bench_unop_wide!(b, size, width => 4, op => ret_self, ty => Matrix2<f32x4>)
    });
    bench_bevy_math_extensions_f32x8!(group, size, |b, size| {
        use bevy_math_extensions::SymmetricMat2x8;
        bench_unop_wide!(b, size, width => 8, op => ret_self, ty => SymmetricMat2x8)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Mat2x8;
        bench_unop_wide!(b, size, width => 8, op => ret_self, ty => Mat2x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::Matrix2;
        use simba::simd::f32x8;
        bench_unop_wide!(b, size, width => 8, op => ret_self, ty => Matrix2<f32x8>)
    });
    group.finish();
}

fn bench_symmetric_matrix2_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar symmetric matrix2 determinant");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_unop!(b, op => determinant, ty => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix2};
        bench_unop!(b, op => determinant, ty => Matrix2<f32>)
    });
    bench_bevy_math_extensions!(group, |b| {
        use bevy_math_extensions::SymmetricMat2;
        bench_unop!(b, op => determinant, ty => SymmetricMat2)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat2;
        bench_unop!(b, op => determinant, ty => Mat2)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_unop!(b, op => determinant, ty => Matrix2<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix2x2::M22;
        use static_math::traits::LinearAlgebra;
        bench_unop!(b, op => det, ty => M22<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat2;
        bench_unop!(b, op => determinant, ty => Mat2<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Matrix2x2F;
        bench_unop!(b, op => det, ty => Matrix2x2F)
    });
    group.finish();
}

fn bench_symmetric_matrix2_determinant_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide symmetric matrix2 determinant");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Mat2;
        bench_unop_wide!(b, size, width => 1, op => determinant, ty => Mat2)
    });
    bench_bevy_math_extensions_f32x4!(group, size, |b, size| {
        use bevy_math_extensions::SymmetricMat2x4;
        bench_unop_wide!(b, size, width => 4, op => determinant, ty => SymmetricMat2x4)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Mat2x4;
        bench_unop_wide!(b, size, width => 4, op => determinant, ty => Mat2x4)
    });
    bench_bevy_math_extensions_f32x8!(group, size, |b, size| {
        use bevy_math_extensions::SymmetricMat2x8;
        bench_unop_wide!(b, size, width => 8, op => determinant, ty => SymmetricMat2x8)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Mat2x8;
        bench_unop_wide!(b, size, width => 8, op => determinant, ty => Mat2x8)
    });
    group.finish();
}

fn bench_symmetric_matrix2_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar symmetric matrix2 inverse");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_unop!(b, op => inverse, ty => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix2};
        bench_unop!(b, op => invert, ty => Matrix2<f32>)
    });
    bench_bevy_math_extensions!(group, |b| {
        use bevy_math_extensions::SymmetricMat2;
        bench_unop!(b, op => inverse, ty => SymmetricMat2)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat2;
        bench_unop!(b, op => inversed, ty => Mat2)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_unop!(b, op => try_inverse, ty => Matrix2<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix2x2::M22;
        use static_math::traits::LinearAlgebra;
        bench_unop!(b, op => inverse, ty => M22<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Matrix2x2F;
        bench_unop!(b, op => inverse, ty => Matrix2x2F)
    });
    group.finish();
}

fn bench_symmetric_matrix2_inverse_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide symmetric matrix2 inverse");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Mat2;
        bench_unop_wide!(b, size, width => 1, op => inverse, ty => Mat2)
    });
    bench_bevy_math_extensions_f32x4!(group, size, |b, size| {
        use bevy_math_extensions::SymmetricMat2x4;
        bench_unop_wide!(b, size, width => 4, op => inverse, ty => SymmetricMat2x4)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Mat2x4;
        bench_unop_wide!(b, size, width => 4, op => inversed, ty => Mat2x4)
    });
    bench_bevy_math_extensions_f32x8!(group, size, |b, size| {
        use bevy_math_extensions::SymmetricMat2x8;
        bench_unop_wide!(b, size, width => 8, op => inverse, ty => SymmetricMat2x8)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Mat2x8;
        bench_unop_wide!(b, size, width => 8, op => inversed, ty => Mat2x8)
    });
    group.finish();
}

fn bench_symmetric_matrix2_mul_symmetric_matrix2(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar symmetric matrix2 mul symmetric matrix2");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_binop!(b, op => mul, ty1 => Mat2, ty2 => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix2;
        bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Matrix2<f32>)
    });
    bench_bevy_math_extensions!(group, |b| {
        use bevy_math_extensions::SymmetricMat2;
        bench_binop!(b, op => mul, ty1 => SymmetricMat2, ty2 => SymmetricMat2)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat2;
        bench_binop!(b, op => mul, ty1 => Mat2, ty2 => Mat2)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Matrix2<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix2x2::M22;
        bench_binop!(b, op => mul, ty1 => M22<f32>, ty2 => M22<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat2;
        bench_binop!(b, op => mul, ty1 => Mat2<f32>, ty2 => Mat2<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Matrix2x2F;
        bench_binop!(b, op => mul, ty1 => Matrix2x2F, ty2 => Matrix2x2F)
    });
    group.finish();
}

fn bench_symmetric_matrix2_mul_symmetric_matrix2_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide symmetric matrix2 mul symmetric matrix2");
    for size in [16, 256].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam_f32x1!(group, size, |b, size| {
            use glam::Mat2;
            bench_binop_wide!(b, size, width => 1, op => mul, ty1 => Mat2, ty2 => Mat2)
        });
        bench_bevy_math_extensions_f32x4!(group, size, |b, size| {
            use bevy_math_extensions::SymmetricMat2x4;
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => SymmetricMat2x4, ty2 => SymmetricMat2x4)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::Mat2x4;
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Mat2x4, ty2 => Mat2x4)
        });
        bench_nalgebra_f32x4!(group, size, |b, size| {
            use nalgebra::Matrix2;
            use simba::simd::f32x4;
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Matrix2<f32x4>, ty2 => Matrix2<f32x4>)
        });
        bench_bevy_math_extensions_f32x8!(group, size, |b, size| {
            use bevy_math_extensions::SymmetricMat2x8;
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => SymmetricMat2x8, ty2 => SymmetricMat2x8)
        });
        bench_ultraviolet_f32x8!(group, size, |b, size| {
            use ultraviolet::Mat2x8;
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Mat2x8, ty2 => Mat2x8)
        });
        bench_nalgebra_f32x8!(group, size, |b, size| {
            use nalgebra::Matrix2;
            use simba::simd::f32x8;
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Matrix2<f32x8>, ty2 => Matrix2<f32x8>)
        });
    }
    group.finish();
}

fn bench_symmetric_matrix2_mul_vector2(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar symmetric matrix2 mul vector2");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::{Mat2, Vec2};
            bench_binop!(b, size, op => mul, ty1 => Mat2, ty2 => Vec2)
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{Matrix2, Vector2};
            bench_binop!(b, size, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
        });
        bench_bevy_math_extensions!(group, size, |b, size| {
            use bevy_math_extensions::{SymmetricMat2, Vec2};
            bench_binop!(b, size, op => mul, ty1 => SymmetricMat2, ty2 => Vec2)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Mat2, Vec2};
            bench_binop!(b, size, op => mul, ty1 => Mat2, ty2 => Vec2)
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Matrix2, Vector2};
            bench_binop!(b, size, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
        });
        bench_static_math!(group, size, |b, size| {
            use static_math::matrix2x2::M22;
            use static_math::vector2::V2;
            bench_binop!(b, size, op => mul, ty1 => M22<f32>, ty2 => V2<f32>)
        });
        bench_vek!(group, size, |b, size| {
            use vek::{Mat2, Vec2};
            bench_binop!(b, size, op => mul, ty1 => Mat2<f32>, ty2 => Vec2<f32>)
        });
        bench_pathfinder!(group, size, |b, size| {
            use pathfinder_geometry::{transform2d::Matrix2x2F, vector::Vector2F};
            bench_binop!(b, size, op => mul, ty1 => Matrix2x2F, ty2 => Vector2F)
        });
    }
    group.finish();
}

fn bench_symmetric_matrix2_mul_vector2_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide symmetric matrix2 mul vector2");
    for size in [16, 256].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam_f32x1!(group, size, |b, size| {
            use glam::{Mat2, Vec2};
            bench_binop_wide!(b, size, width => 1, op => mul, ty1 => Mat2, ty2 => Vec2)
        });
        bench_bevy_math_extensions_f32x4!(group, size, |b, size| {
            use bevy_math_extensions::{SymmetricMat2x4, Vec2x4};
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => SymmetricMat2x4, ty2 => Vec2x4)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{Mat2x4, Vec2x4};
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Mat2x4, ty2 => Vec2x4)
        });
        bench_nalgebra_f32x4!(group, size, |b, size| {
            use nalgebra::{Matrix2, Vector2};
            use simba::simd::f32x4;
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Matrix2<f32x4>, ty2 => Vector2<f32x4>)
        });
        bench_bevy_math_extensions_f32x8!(group, size, |b, size| {
            use bevy_math_extensions::{SymmetricMat2x8, Vec2x8};
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => SymmetricMat2x8, ty2 => Vec2x8)
        });
        bench_ultraviolet_f32x8!(group, size, |b, size| {
            use ultraviolet::{Mat2x8, Vec2x8};
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Mat2x8, ty2 => Vec2x8)
        });
        bench_nalgebra_f32x8!(group, size, |b, size| {
            use nalgebra::{Matrix2, Vector2};
            use simba::simd::f32x8;
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Matrix2<f32x8>, ty2 => Vector2<f32x8>)
        });
    }
    group.finish();
}

criterion_group!(
    symmetric_matrix2_benches,
    bench_symmetric_matrix2_ret_self,
    bench_symmetric_matrix2_ret_self_wide,
    bench_symmetric_matrix2_determinant,
    bench_symmetric_matrix2_determinant_wide,
    bench_symmetric_matrix2_inverse,
    bench_symmetric_matrix2_inverse_wide,
    bench_symmetric_matrix2_mul_symmetric_matrix2,
    bench_symmetric_matrix2_mul_symmetric_matrix2_wide,
    bench_symmetric_matrix2_mul_vector2,
    bench_symmetric_matrix2_mul_vector2_wide,
);
criterion_main!(symmetric_matrix2_benches);
