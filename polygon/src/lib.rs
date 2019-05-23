mod arc_tool;
mod grad_analy;

pub use arc_tool::*;
pub use grad_analy::*;

//将一个矩形转化为圆角矩形， 返回多边形的顶点流， 顶点为三维顶点
pub fn split_by_radius(x: f32, y: f32, w: f32, h: f32, radius: f32, z: f32) -> Vec<f32> {
    unimplemented!()
}

//将一个多边形按照线性渐变颜色切分， 返回新的多边形
pub fn split_by_lg(polygon: Vec<f32>, lg_pos: &[f32], start: (f32, f32), end: (f32, f32)) -> (Vec<f32>, Vec<u16>) {
    unimplemented!()
}

pub struct LgCfg{
    pub unit: usize,
    pub data: Vec<f32>,
}
//将属性流按照线性渐变插值,  返回插值后的属性流
pub fn interp_by_lg(polygon: &[f32], lg_attrs: Vec<LgCfg>, lg_pos: &[f32], start: (f32, f32), end: (f32, f32)) -> Vec<Vec<f32>> {
    unimplemented!()
}

//将多边形转换为三角形
pub fn to_triangle(indeices: &[u16]) -> Vec<u16> {
    unimplemented!()
}