@compute
@workgroup_size(64, 1, 1) 
fn comp_main(@builtin(global_invocation_id) param: vec3<u32>) {
    return;
}
