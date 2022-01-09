easy_impl! {
    Vec2 mint::Vector2<f32> { x, y },
    Vec3 mint::Vector3<f32> { x, y, z },
    Vec4 mint::Vector4<f32> { x, y, z, w },

    Vec2 mint::Point2<f32> { x, y },
    Vec3 mint::Point3<f32> { x, y, z },

    IVec2 mint::Vector2<i32> { x, y },
    IVec3 mint::Vector3<i32> { x, y, z },
    IVec4 mint::Vector4<i32> { x, y, z, w },

    IVec2 mint::Point2<i32> { x, y },
    IVec3 mint::Point3<i32> { x, y, z },

    UVec2 mint::Vector2<u32> { x, y },
    UVec3 mint::Vector3<u32> { x, y, z },
    UVec4 mint::Vector4<u32> { x, y, z, w },

    UVec2 mint::Point2<u32> { x, y },
    UVec3 mint::Point3<u32> { x, y, z },

    BVec2 mint::Vector2<bool> { x, y },
    BVec3 mint::Vector3<bool> { x, y, z },
    BVec4 mint::Vector4<bool> { x, y, z, w },

    BVec2 mint::Point2<bool> { x, y },
    BVec3 mint::Point3<bool> { x, y, z },

    DVec2 mint::Vector2<f64> { x, y },
    DVec3 mint::Vector3<f64> { x, y, z },
    DVec4 mint::Vector4<f64> { x, y, z, w },

    DVec2 mint::Point2<f64> { x, y },
    DVec3 mint::Point3<f64> { x, y, z },

    Mat2 mint::ColumnMatrix2<f32> { x, y },
    Mat3 mint::ColumnMatrix3<f32> { x, y, z },
    Mat4 mint::ColumnMatrix4<f32> { x, y, z, w },

    DMat2 mint::ColumnMatrix2<f64> { x, y },
    DMat3 mint::ColumnMatrix3<f64> { x, y, z },
    DMat4 mint::ColumnMatrix4<f64> { x, y, z, w },
}
