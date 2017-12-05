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

            fn width(&self) -> f32 {
                self.base.asset.width() as f32
            }
            fn height(&self) -> f32 {
                self.base.asset.height() as f32
            }
            fn half_width(&self) -> f32 {
                self.base.asset.half_width()
            }
            fn half_height(&self) -> f32 {
                self.base.asset.half_height()
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
				return self.base.rvel
			}

            fn facing(&self) -> f32 {
                return self.base.facing
            }
            fn set_facing(&mut self, facing: f32) {
                self.base.facing = facing
            }

            fn bbox_size(&self) -> f32 {
                return self.base.bbox_size
            }

            fn life(&self) -> f32 {
                return self.base.life
            }
            fn set_life(&mut self, life: f32) {
                self.base.life = life
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

            fn draw(&self, ctx: &mut Context, world_coords: (u32, u32)) {
                self.base.asset.draw(ctx, world_coords, self.position(), self.facing())
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

            fn draw(&self, ctx: &mut Context, world_coords: (u32, u32)) {
                let screen_right = world_coords.0 as f32;
                let pos = self.position();

                if pos.x < self.half_width() {

                    let wrap_pos = Point2::new(pos.x + screen_right, pos.y);
                    self.base.asset.draw(ctx, world_coords, wrap_pos, self.facing());

                } else if pos.x > (screen_right - self.half_width()) {

                    let wrap_pos = Point2::new(pos.x - screen_right, pos.y);
                    self.base.asset.draw(ctx, world_coords, wrap_pos, self.facing());
                }

                self.base.asset.draw(ctx, world_coords, self.position(), self.facing())
            }
        }
    }
}