use bevy::prelude::*;
use rhai::Engine;
use std::ops::*;

macro_rules! register_float_vecn {
    ($engine:ident, $vec_type:ident,$scalar:ident,3) => {
        $engine
            .register_fn("distance", $vec_type::distance)
            .register_fn("distance_squared", $vec_type::distance_squared)
            .register_fn("dot", $vec_type::dot)
            .register_fn("cross", $vec_type::cross);
    };
    ($engine:ident, $vec_type:ident,$scalar:ident,2) => {
        $engine
            .register_fn("distance", $vec_type::distance)
            .register_fn("distance_squared", $vec_type::distance_squared)
            .register_fn("dot", $vec_type::dot)
    };
}

macro_rules! register_vecn {
    ($engine:ident, $vec_type:ident,$scalar:ident) => {
        $engine
            .register_type::<$vec_type>()
            //Instantiating
            .register_fn("new", $vec_type::new)
            .register_fn("default", $vec_type::default)
            //Math
            .register_fn("add", $vec_type::add)
            .register_fn("add_assign", $vec_type::add_assign)
            .register_fn("sub", $vec_type::sub)
            .register_fn("sub_assign", $vec_type::sub_assign)
            .register_fn("mul", <$vec_type as Mul<$scalar>>::mul)
            .register_fn("mul_assign", <$vec_type as MulAssign<$scalar>>::mul_assign)
            .register_fn("mul", <$vec_type as Mul<$vec_type>>::mul)
            .register_fn(
                "mul_assign",
                <$vec_type as MulAssign<$vec_type>>::mul_assign,
            )
            .register_fn("div", <$vec_type as Div<$scalar>>::div)
            .register_fn("div_assign", <$vec_type as DivAssign<$scalar>>::div_assign)
            .register_fn("div", <$vec_type as Div<$vec_type>>::div)
            .register_fn(
                "div_assign",
                <$vec_type as DivAssign<$vec_type>>::div_assign,
            )
            //Convenience
            .register_fn("extend", $vec_type::extend)
            .register_fn("to_array", $vec_type::to_array);
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
    };
}
macro_rules! register_vec2 {
    ($engine:ident, $vec_type:ident,$scalar:ident) => {
        register_vecn!($engine, $vec_type, $scalar);
        register_getters_and_setters_vec2!($engine, $vec_type, $scalar);
    };
}

pub fn register_bevy_math_types(engine: &mut Engine) {
    register_vec3!(engine, Vec3, f32);
    register_float_vecn!(engine, Vec3, f32, 3);
    register_signed!(engine, Vec3, f32);
    register_vec3!(engine, IVec3, i32);
    register_vecn_conversion_to_f32!(engine, IVec3, i32);
    register_signed!(engine, IVec3, i32);
    register_vec3!(engine, UVec3, u32);
    register_vecn_conversion_to_f32!(engine, UVec3, u32);

    register_vec2!(engine, Vec2, f32);
    register_float_vecn!(engine, Vec2, f32, 2);
    register_signed!(engine, Vec2, f32);
    register_vec2!(engine, IVec2, i32);
    register_vecn_conversion_to_f32!(engine, IVec2, i32);
    register_signed!(engine, IVec2, i32);
    register_vec2!(engine, UVec2, u32);
    register_vecn_conversion_to_f32!(engine, UVec2, u32);
}
