struct TestData {
    two: vec2<f32>;
};

[[block]]
struct INPUT {
    in_data: TestData;
};

[[block]]
struct OUTPUT {
    out_data: TestData;
};

[[group(0), binding(0)]]
var<storage> global: INPUT;
[[group(0), binding(1)]]
var<storage, read_write> global1: OUTPUT;

fn main1() {
    let e4: TestData = global.in_data;
    global1.out_data = e4;
    return;
}

[[stage(compute), workgroup_size(1, 1, 1)]]
fn main() {
    main1();
    return;
}
