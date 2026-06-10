use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkShaderStageFlag`]s
    ///
    /// # Description
    /// [`VkShaderStageFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkShaderStageFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkShaderStageFlags;

    /// Bitmask specifying a pipeline stage
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkShaderStageFlag {
        /// [`VkShaderStageFlag::Vertex`] specifies the vertex stage.
        Vertex = 0x00000001,

        /// [`VkShaderStageFlag::TessellationControl`] specifies the tessellation control stage.
        TessellationControl = 0x00000002,

        /// [`VkShaderStageFlag::TessellationEvaluation`] specifies the tessellation evaluation
        /// stage.
        TessellationEvaluation = 0x00000004,

        /// [`VkShaderStageFlag::Geometry`] specifies the geometry stage.
        Geometry = 0x00000008,

        /// [`VkShaderStageFlag::Fragment`] specifies the fragment stage.
        Fragment = 0x00000010,

        /// [`VkShaderStageFlag::Compute`] specifies the compute stage.
        Compute = 0x00000020,

        /// [`VkShaderStageFlag::AllGraphics`] is a combination of bits used as shorthand to
        /// specify all graphics stages defined above (excluding the compute stage).
        AllGraphics = 0x0000001F,

        /// [`VkShaderStageFlag::All`] is a combination of bits used as shorthand to specify all
        /// shader stages supported by the device, including all additional stages which are
        /// introduced by extensions.
        All = 0x7FFFFFFF,

        /// [`VkShaderStageFlag::RaygenKhr`] specifies the ray generation stage.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        RaygenKhr = 0x00000100,

        /// [`VkShaderStageFlag::AnyHitKhr`] specifies the any-hit stage.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        AnyHitKhr = 0x00000200,

        /// [`VkShaderStageFlag::ClosestHitKhr`] specifies the closest hit stage.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        ClosestHitKhr = 0x00000400,

        /// [`VkShaderStageFlag::MissKhr`] specifies the miss stage.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        MissKhr = 0x00000800,

        /// [`VkShaderStageFlag::IntersectionKhr`] specifies the intersection stage.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        IntersectionKhr = 0x00001000,

        /// [`VkShaderStageFlag::CallableKhr`] specifies the callable stage.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        CallableKhr = 0x00002000,

        /// [`VkShaderStageFlag::TaskExt`] specifies the task stage.
        ///
        /// Provided by [`ext_mesh_shader`]
        TaskExt = 0x00000040,

        /// [`VkShaderStageFlag::MeshExt`] specifies the mesh stage.
        ///
        /// Provided by [`ext_mesh_shader`]
        MeshExt = 0x00000080,

        /// Provided by [`huawei_subpass_shading`]
        SubpassShadingHuawei = 0x00004000,

        /// [`VkShaderStageFlag::ClusterCullingHuawei`] specifies the cluster culling stage.
        ///
        /// Provided by [`huawei_cluster_culling_shader`]
        ClusterCullingHuawei = 0x00080000,
    }
}
