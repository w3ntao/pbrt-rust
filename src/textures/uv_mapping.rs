use crate::pbrt::*;

pub struct UVMapping {
    //Float su, sv, du, dv;
    su: Float,
    sv: Float,
    du: Float,
    dv: Float,
}

impl UVMapping {
    pub fn new(su: Float, sv: Float, du: Float, dv: Float) -> Self {
        return Self { su, sv, du, dv };
    }
}

impl TextureMapping2D for UVMapping {
    fn map(&self, ctx: &TextureEvalContext) -> TexCoord2D {
        // Compute texture differentials for 2D $(u,v)$ mapping

        let dsdx = self.su * ctx.dudx;
        let dsdy = self.su * ctx.dudy;
        let dtdx = self.sv * ctx.dvdx;
        let dtdy = self.sv * ctx.dvdy;

        let st = Point2f {
            x: self.su * ctx.uv.x + self.du,
            y: self.sv * ctx.uv.y + self.dv,
        };

        return TexCoord2D {
            st,
            dsdx,
            dsdy,
            dtdx,
            dtdy,
        };
    }
}
