#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ObjectType {
    Integer,
    Real,
    Vector
}


#[derive(PartialEq, Debug)]
pub enum Object {
    Integer(Integer),
    Real(Real),
    Vector(Vector)
}

impl Object {
    pub fn object_type(self: &Self) -> ObjectType {
        match self {
            Object::Integer(_) => return ObjectType::Integer,
            Object::Real(_) => return ObjectType::Real,
            Object::Vector(_) => return ObjectType::Vector
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Integer {
    pub value: i64
}

#[derive(PartialEq, Debug)]
pub struct Real {
    pub value: f64
}

#[derive(PartialEq, Debug)]
pub struct Vector {
    pub components: Vec<Object>,
    pub object_type: ObjectType
}

pub enum ConversionFailure {
    Error
}

pub trait ConvertFromString: Sized {
    type ErrorType: Sized;
    fn new(value: &str) -> Result<Self, Self::ErrorType>;
}

impl ConvertFromString for Integer {
    type ErrorType = ConversionFailure;
    fn new(value: &str) -> Result<Self, ConversionFailure> {
        return value.parse::<i64>()
            .map(|number| Integer { value: number })
            .map_err(|_| ConversionFailure::Error);
    }
}


impl ConvertFromString for Real {
    type ErrorType = ConversionFailure;
    fn new(value: &str) -> Result<Self, ConversionFailure> {
        return value.parse::<f64>()
            .map(|number| Real { value: number })
            .map_err(|_| ConversionFailure::Error);
    }
}

impl ConvertFromString for Vector {
    type ErrorType = ConversionFailure;
    fn new(_: &str) -> Result<Self, ConversionFailure> {
        return Result::Err(ConversionFailure::Error);
    }
}

#[derive(PartialEq)]
pub struct AnyValue<'a> {
    pub object_type: Option<ObjectType>,
    pub text: &'a str
}

impl AnyValue<'_> {
    pub fn convert(self: &Self, target_type: ObjectType) -> Result<Object, ConversionFailure> {
        match target_type {
            ObjectType::Integer => Integer::new(self.text).map(Object::Integer),
            ObjectType::Real => Real::new(self.text).map(Object::Real),
            ObjectType::Vector => Vector::new(self.text).map(Object::Vector)
        }
    }
}

pub struct Parameter<'a> {
    pub name: &'a str,
    pub expected_type: ObjectType
}
pub trait Callable<'a> {
    fn parameters(self: &Self) -> Vec<Parameter<'static>>;
    fn callback(self: &Self, parameters: Vec<Object>) -> Vec<Object>;
}

pub trait CallableExtension {
    fn call(self: &Self, parameters: Vec<Object>) -> Result<Vec<Object>, CallError>;
}

pub enum CallError {
    TypeError
}

impl<'a, T> CallableExtension for T where T: Callable<'a> {
    fn call(self: &Self, parameters: Vec<Object>) -> Result<Vec<Object>, CallError> {
        let lhs: Vec<ObjectType> = parameters.iter().map(Object::object_type).collect();
        let rhs: Vec<ObjectType> = self.parameters().iter().map(|v| v.expected_type).collect();
        if  lhs == rhs {
            return Result::Err(CallError::TypeError);
        } else {
            return Result::Ok(self.callback(parameters));
        }
    }
}

pub struct PrimitiveFunction<'a, T> where T: Fn(f64) -> f64 {
    pub name: &'a str,
    pub function: T
}

impl<'a, T> Callable<'a> for PrimitiveFunction<'a, T> where T: Fn(f64) -> f64 {
    fn parameters(self: &Self) -> Vec<Parameter<'static>> {
        vec![Parameter { name: "arg0", expected_type: ObjectType::Real }]
    }

    fn callback(self: &Self, parameters: Vec<Object>) -> Vec<Object> {
        if let Object::Real(number) = parameters.first().unwrap() {
            return vec![Object::Real(Real { value: (self.function)(number.value) })]
        } else {
            panic!();
        }
    }
}