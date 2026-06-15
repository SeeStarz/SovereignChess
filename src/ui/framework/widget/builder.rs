use crate::{
    geometry::{FPosition, FSize, Size},
    ui::framework::{
        layout::{
            AxisSizingRequest::{Expand, Fixed, Shrink},
            CardinalAnchor, FlexAnchor, FlexDirection,
            Positioning::{self, Absolute, Offset, Relative},
        },
        widget::prototype::{
            InputHandler, RenderFunction, SizeRequest, Spec, SpecNode, ignore_input, no_render,
        },
    },
};

pub struct InnerData {
    pub children: Vec<SpecNode>,
    pub size_request: SizeRequest,
    pub flex_direction: FlexDirection,
    pub positioning: Positioning,
    pub child_origin: CardinalAnchor,
    pub input_handler: InputHandler,
    pub render_function: RenderFunction,
}

pub struct Builder {
    pub inner_data: InnerData,
}

impl Builder {
    pub fn finalize(self) -> SpecNode {
        SpecNode {
            children: self.inner_data.children,
            core: Spec {
                size_request: self.inner_data.size_request,
                flex_direction: self.inner_data.flex_direction,
                positioning: self.inner_data.positioning,
                child_origin: self.inner_data.child_origin,
                input_handler: self.inner_data.input_handler,
                render_function: self.inner_data.render_function,
            },
        }
    }
}

//////////////////
// INITIALIZERS //
//////////////////

impl Builder {
    pub fn default() -> Self {
        Builder {
            inner_data: InnerData {
                children: Vec::new(),
                size_request: Size::new(Fixed(0.0), Fixed(0.0)),
                flex_direction: FlexDirection::Right,
                positioning: Positioning::Offset(FPosition::default(), FlexAnchor::LeftOrTop),
                child_origin: CardinalAnchor::TopLeft,
                input_handler: Box::new(ignore_input),
                render_function: Box::new(no_render),
            },
        }
    }

    pub fn new_flex(direction: FlexDirection) -> Self {
        Self::default().flex(direction).shrink()
    }

    pub fn new_row(right: bool) -> Self {
        if right {
            Self::new_flex(FlexDirection::Right)
        } else {
            Self::new_flex(FlexDirection::Left)
        }
    }

    pub fn new_col(down: bool) -> Self {
        if down {
            Self::new_flex(FlexDirection::Down)
        } else {
            Self::new_flex(FlexDirection::Up)
        }
    }
}

///////////////////////
// HELPFUL FUNCTIONS //
///////////////////////

impl Builder {
    pub fn shrink(self) -> Self {
        self.size_request(Size::new(Shrink, Shrink))
    }

    pub fn expand(self) -> Self {
        self.size_request(Size::new(Expand(1), Expand(1)))
    }

    pub fn size(self, size: FSize) -> Self {
        self.size_request(Size::new(Fixed(size.width), Fixed(size.height)))
    }

    pub fn width(mut self, width: f32) -> Self {
        self.inner_data.size_request.width = Fixed(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.inner_data.size_request.height = Fixed(height);
        self
    }

    pub fn row(self, right: bool) -> Self {
        if right {
            self.flex(FlexDirection::Right)
        } else {
            self.flex(FlexDirection::Left)
        }
    }

    pub fn col(self, down: bool) -> Self {
        if down {
            self.flex(FlexDirection::Down)
        } else {
            self.flex(FlexDirection::Up)
        }
    }

    pub fn fixed(self, position: FPosition, anchor: CardinalAnchor) -> Self {
        self.position(Absolute(position, anchor))
    }

    pub fn relative(self, offset: FPosition, anchor: CardinalAnchor) -> Self {
        self.position(Relative(offset, anchor))
    }

    pub fn offset(self, offset: FPosition, anchor: FlexAnchor) -> Self {
        self.position(Offset(offset, anchor))
    }
}

///////////////////
// BASIC BUILDER //
///////////////////

impl Builder {
    pub fn children(mut self, children: Vec<SpecNode>) -> Self {
        self.inner_data.children = children;
        self
    }

    pub fn size_request(mut self, size_request: SizeRequest) -> Self {
        self.inner_data.size_request = size_request;
        self
    }

    pub fn flex(mut self, flex_direction: FlexDirection) -> Self {
        self.inner_data.flex_direction = flex_direction;
        self
    }

    pub fn position(mut self, positioning: Positioning) -> Self {
        self.inner_data.positioning = positioning;
        self
    }

    pub fn child_origin(mut self, child_origin: CardinalAnchor) -> Self {
        self.inner_data.child_origin = child_origin;
        self
    }

    pub fn input(mut self, input_handler: InputHandler) -> Self {
        self.inner_data.input_handler = input_handler;
        self
    }

    pub fn render(mut self, render_function: RenderFunction) -> Self {
        self.inner_data.render_function = render_function;
        self
    }
}
