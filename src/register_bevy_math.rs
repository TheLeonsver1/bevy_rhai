use bevy::prelude::*;
use rhai::Engine;
use std::ops::*;

macro_rules! register_float_vecn {
    ($engine:ident, $vec_type:ident,$scalar:ident) => {
        $engine
            .register_fn("distance", $vec_type::distance)
            .register_fn("distance_squared", $vec_type::distance_squared)
            .register_fn("dot", $vec_type::dot)
            .register_fn("normalize", $vec_type::normalize)
            .register_fn("normalize_or_zero", $vec_type::normalize_or_zero)
            .register_fn("is_normalized", $vec_type::is_normalized)
            .register_fn("recip", $vec_type::recip)
            .register_fn("angle_between", $vec_type::angle_between)
            .register_fn("lerp", $vec_type::lerp)
            .register_fn("length_recip", $vec_type::length_recip)
            .register_fn("clamp", $vec_type::clamp)
            .register_fn("ceil", $vec_type::ceil)
            .register_fn("floor", $vec_type::floor)
            .register_fn("exp", $vec_type::exp)
            .register_fn("clamp_length", $vec_type::clamp_length)
            .register_fn("clamp_length_max", $vec_type::clamp_length_max)
            .register_fn("clamp_length_min", $vec_type::clamp_length_min)
    };
}

macro_rules! register_vecn {
    ($engine:ident, $vec_type:ident,$scalar:ident) => {
        $engine.register_type::<$vec_type>();
        //Instantiating;
        $engine.register_fn("new", $vec_type::new);
        $engine.register_fn("default", $vec_type::default);
        //Math;
        $engine.register_fn("add", $vec_type::add);
        $engine.register_fn("add_assign", $vec_type::add_assign);
        $engine.register_fn("sub", $vec_type::sub);
        $engine.register_fn("sub_assign", $vec_type::sub_assign);
        $engine.register_fn("mul", <$vec_type as Mul<$scalar>>::mul);
        $engine.register_fn("mul_assign", <$vec_type as MulAssign<$scalar>>::mul_assign);
        $engine.register_fn("mul", <$vec_type as Mul<$vec_type>>::mul);
        $engine.register_fn(
            "mul_assign",
            <$vec_type as MulAssign<$vec_type>>::mul_assign,
        );

        $engine.register_fn("div", <$vec_type as Div<$scalar>>::div);
        $engine.register_fn("div_assign", <$vec_type as DivAssign<$scalar>>::div_assign);
        $engine.register_fn("div", <$vec_type as Div<$vec_type>>::div);
        $engine.register_fn(
            "div_assign",
            <$vec_type as DivAssign<$vec_type>>::div_assign,
        );
        //Convenience
        $engine.register_fn("to_array", $vec_type::to_array);
        $engine.register_fn("splat", $vec_type::splat);
        //  (TODO:i don't register Vec4s atm, this would be meaningless on them since they won't be possible to use)
        $engine.register_fn("max", $vec_type::max);
        $engine.register_fn("max_element", $vec_type::max_element);
        $engine.register_fn("min", $vec_type::min);
        $engine.register_fn("min", $vec_type::min_element);


        //Comparing
        $engine.register_fn("cmple", $vec_type::cmple);
        $engine.register_fn("cmplt", $vec_type::cmplt);
        $engine.register_fn("cmpge", $vec_type::cmpge);
        $engine.register_fn("cmpgt", $vec_type::cmpgt);
        $engine.register_fn("cmpeq", $vec_type::cmpeq);
        $engine.register_fn("cmpne", $vec_type::cmpne);

    };
}
macro_rules! register_vecn_conversion_to_f32 {
    ($engine:ident, $vec_type:ident,$scalar:ident) => {
        $engine.register_fn("as_f32", $vec_type::as_f32);
    };
}
macro_rules! register_getters_and_setters_vec2 {
    ($engine:ident, $vec_type:ident,$scalar:ident) => {
        $engine.register_get_set(
            "x",
            |vec: &mut $vec_type| vec.x,
            |vec: &mut $vec_type, new_val: $scalar| vec.x = new_val,
        );
        $engine.register_get_set(
            "y",
            |vec: &mut $vec_type| vec.y,
            |vec: &mut $vec_type, new_val: $scalar| vec.y = new_val,
        );
    };
}
macro_rules! complete_getters_and_setters_to_vec3 {
    ($engine:ident, $vec_type:ident,$scalar:ident) => {
        $engine.register_get_set(
            "z",
            |vec: &mut $vec_type| vec.z,
            |vec: &mut $vec_type, new_val: $scalar| vec.z = new_val,
        );
    };
}
macro_rules! register_signed {
    ($engine:ident, $vec_type:ident,$scalar:ident) => {
        $engine
            .register_fn("signum", $vec_type::signum)
            .register_fn("neg", $vec_type::neg);
    };
}
macro_rules! register_vec3 {
    ($engine:ident, $vec_type:ident,$scalar:ident) => {
        register_vecn!($engine, $vec_type, $scalar);
        register_getters_and_setters_vec2!($engine, $vec_type, $scalar);
        complete_getters_and_setters_to_vec3!($engine, $vec_type, $scalar);
        $engine.register_fn("truncate", $vec_type::truncate)
    };
}
macro_rules! register_vec2 {
    ($engine:ident, $vec_type:ident,$scalar:ident) => {
        register_vecn!($engine, $vec_type, $scalar);
        register_getters_and_setters_vec2!($engine, $vec_type, $scalar);
        $engine.register_fn("extend", $vec_type::extend);
    };
}

macro_rules! register_bvecn {
    ($engine:ident, $vec_type:ident) => {
        $engine.register_type::<$vec_type>();
        $engine.register_fn("all", $vec_type::all);
        $engine.register_fn("any", $vec_type::any);
    };
}

pub fn register_bevy_math_types(engine: &mut Engine) {
    register_vec3!(engine, Vec3, f32);
    register_float_vecn!(engine, Vec3, f32);
    //Vec2 doesn't have cross
    engine.register_fn("cross", Vec3::cross);
    register_signed!(engine, Vec3, f32);
    register_vec3!(engine, IVec3, i32);
    register_vecn_conversion_to_f32!(engine, IVec3, i32);
    register_signed!(engine, IVec3, i32);
    register_vec3!(engine, UVec3, u32);
    register_vecn_conversion_to_f32!(engine, UVec3, u32);

    register_vec2!(engine, Vec2, f32);
    register_float_vecn!(engine, Vec2, f32);
    register_signed!(engine, Vec2, f32);
    register_vec2!(engine, IVec2, i32);
    register_vecn_conversion_to_f32!(engine, IVec2, i32);
    register_signed!(engine, IVec2, i32);
    register_vec2!(engine, UVec2, u32);
    register_vecn_conversion_to_f32!(engine, UVec2, u32);

    engine.register_type::<Quat>();
    //Instantiating
    engine.register_fn("from_xyzw", Quat::from_xyzw);
    engine.register_fn("default", Quat::default);
    //Math
    engine.register_fn("add", Quat::add);
    engine.register_fn("sub", Quat::sub);
    engine.register_fn("mul", <Quat as Mul<f32>>::mul);
    engine.register_fn("div", <Quat as Div<f32>>::div);
    engine.register_fn("mul_quat", Quat::mul_quat);
    engine.register_fn("angle_between", Quat::angle_between);
    engine.register_fn("conjugate", Quat::conjugate);
    engine.register_fn("dot", Quat::dot);
    engine.register_fn("from_euler", Quat::from_euler);
    engine.register_fn("length", Quat::length);
    engine.register_fn("length_squared", Quat::length_squared);
    engine.register_fn("normalize", Quat::normalize);
    engine.register_fn("is_normalized", Quat::is_normalized);
    engine.register_fn("to_euler", Quat::to_euler);
    engine.register_fn("to_axis_angle", Quat::to_axis_angle);

    register_bvecn!(engine, BVec3);
    register_bvecn!(engine, BVec2);
}
