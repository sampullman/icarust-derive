#![recursion_limit = "1024"]
extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Actor)]
pub fn actor(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_actor(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_actor(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Actor for #name {

            fn alive(&self) -> bool {
                self.base.alive
            }
            fn kill(&mut self) {
                self.base.alive = false
            }

            fn width(&self, ctx: &mut Context) -> f32 {
                self.base.asset.width(ctx) as f32
            }
            fn height(&self, ctx: &mut Context) -> f32 {
                self.base.asset.height(ctx) as f32
            }
            fn half_width(&self, ctx: &mut Context) -> f32 {
                self.base.asset.half_width(ctx)
            }
            fn half_height(&self, ctx: &mut Context) -> f32 {
                self.base.asset.half_height(ctx)
            }
            fn center(&self, ctx: &mut Context) -> Point2 {
                Point2::new(self.x() + self.half_height(ctx), self.y() + self.half_height(ctx))
            }

            fn position(&self) -> Point2 {
                self.base.pos
            }
            fn set_position(&mut self, pos: Point2) {
                self.base.pos = pos
            }
			fn add_position(&mut self, pos: Point2) {
				self.base.pos.x += pos.x;
				self.base.pos.y += pos.y
			}
            fn x(&self) -> f32 {
                self.base.pos.x
            }
            fn y(&self) -> f32 {
                self.base.pos.y
            }
            fn set_x(&mut self, x: f32) {
                self.base.pos.x = x
            }
            fn set_y(&mut self, y: f32) {
                self.base.pos.y = y
            }

            fn velocity(&self) -> Vector2 {
                self.base.velocity.clone()
            }
            fn set_velocity_xy(&mut self, x: f32, y: f32) {
                self.base.velocity.x = x;
                self.base.velocity.y = y
            }
            fn set_velocity(&mut self, vel: Vector2) {
                self.base.velocity = vel;
            }
			fn rvel(&self) -> f32 {
				self.base.rvel
			}

            fn facing(&self) -> f32 {
                self.base.facing
            }
            fn set_facing(&mut self, facing: f32) {
                self.base.facing = facing
            }

            fn bbox_size(&self) -> f32 {
                self.base.bbox_size
            }

            fn physics_id(&self) -> PhysicsId {
                self.base.physics_id
            }
            fn add_to_world(&mut self, world: &mut CollisionWorld2, id: PhysicsId) {
                self.base.physics_id = id;
                //let pos = Isometry2::new(Vector2::)
                //world.deferred_add(id, )
            }
        }
    }
}

#[proc_macro_derive(Widget)]
pub fn widget(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_widget(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_widget(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Widget for #name {

            fn position(&self) -> Point2 {
                return self.base.pos
            }
            fn set_position(&mut self, pos: Point2) {
                self.base.pos = pos
            }

            fn facing(&self) -> f32 {
                return self.base.facing
            }
            fn set_facing(&mut self, facing: f32) {
                self.base.facing = facing
            }

            fn width(&self) -> u32 {
                self.base.asset.text.width()
            }
            fn height(&self) -> u32 {
                self.base.asset.text.height()
            }

            fn half_width(&self) -> f32 {
                self.base.asset.text.width() as f32 / 2.0
            }
            fn half_height(&self) -> f32 {
                self.base.asset.text.height() as f32 / 2.0
            }
        }
    }
}

#[proc_macro_derive(Drawable)]
pub fn drawable(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_drawable(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_drawable(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Drawable for #name {

            fn draw(&self, ctx: &mut Context, camera: &Camera) {
                crate::util::draw_asset(ctx, &self.base.asset, self.position(), self.facing(), camera)
            }
        }
    }
}

#[proc_macro_derive(WrappedDrawable)]
pub fn wrapped_drawable(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_wrapped_drawable(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_wrapped_drawable(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Drawable for #name {

            fn draw(&self, ctx: &mut Context, camera: &Camera) {
                let screen_right = camera.world_width();
                let pos = self.position();

                if pos.x < self.half_width() {

                    let wrap_pos = Point2::new(pos.x + screen_right, pos.y);
                    crate::util::draw_asset(ctx, &self.base.asset, wrap_pos, self.facing(), camera)

                } else if pos.x > (screen_right - self.half_width()) {

                    let wrap_pos = Point2::new(pos.x - screen_right, pos.y);
                    crate::util::draw_asset(ctx, &self.base.asset, wrap_pos, self.facing(), camera)
                }

                crate::util::draw_asset(ctx, &self.base.asset, self.position(), self.facing(), camera)
            }
        }
    }
}