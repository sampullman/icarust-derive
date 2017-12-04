#![recursion_limit = "128"]
extern crate proc_macro;
extern crate syn;
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

            fn draw(&self, ctx: &mut Context, world_coords: (u32, u32)) {
                self.actor.asset.draw(ctx, world_coords, self.position(), self.facing())
            }

            fn position(&self) -> Point2 {
                return self.actor.pos
            }
            fn set_position(&mut self, pos: Point2) {
                self.actor.pos = pos
            }
			fn add_position(&mut self, pos: Point2) {
				self.actor.pos.x += pos.x;
				self.actor.pos.y += pos.y
			}
            fn x(&self) -> f32 {
                return self.actor.pos.x
            }
            fn y(&self) -> f32 {
                return self.actor.pos.y
            }
            fn set_x(&mut self, x: f32) {
                self.actor.pos.x = x
            }
            fn set_y(&mut self, y: f32) {
                self.actor.pos.y = y
            }

            fn velocity(&self) -> Vector2 {
                return self.actor.velocity.clone()
            }
            fn set_velocity_xy(&mut self, x: f32, y: f32) {
                self.actor.velocity.x = x;
                self.actor.velocity.y = y
            }
            fn set_velocity(&mut self, vel: Vector2) {
                self.actor.velocity = vel;
            }
			fn rvel(&self) -> f32 {
				return self.actor.rvel
			}

            fn facing(&self) -> f32 {
                return self.actor.facing
            }
            fn set_facing(&mut self, facing: f32) {
                self.actor.facing = facing
            }

            fn bbox_size(&self) -> f32 {
                return self.actor.bbox_size
            }

            fn life(&self) -> f32 {
                return self.actor.life
            }
            fn set_life(&mut self, life: f32) {
                self.actor.life = life
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

            fn draw(&self, ctx: &mut Context, world_coords: (u32, u32)) {
                self.base.asset.draw(ctx, world_coords, self.position(), self.facing())
            }

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
        }
    }
}