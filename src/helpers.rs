use super::*;

impl Object {
    /// Returns `true` if self has another object.
    pub fn has(&self, obj: Object) -> bool {self.matches(&has(obj))}
    /// Returns `true` if self has not another object.
    pub fn has_not(&self, obj: Object) -> bool {self.matches(&has_not(obj))}
    /// Returns `true` if self is on another object.
    pub fn is_on(&self, obj: Object) -> bool {
        self.matches(&on(obj).into())
    }
    /// Returns `true` if self leans toward another object.
    pub fn is_leaning_toward(&self, obj: Object) -> bool {
        self.matches(&lean_toward(obj).into())
    }
    /// Returns `true` if self is in another object.
    pub fn is_in(&self, obj: Object) -> bool {
        self.matches(&in_(obj).into())
    }
    /// Returns `true` if self is out of another object.
    pub fn is_out_of(&self, obj: Object) -> bool {
        self.matches(&out_of(obj).into())
    }
    /// Returns `true` if self was killed by another object.
    pub fn was_killed_by(&self, obj: Object) -> bool {self.matches(&killed_by(obj))}
    /// Returns `true` if self killed another object.
    pub fn killed(&self, obj: Object) -> bool {self.matches(&killed(obj))}
    /// Returns `true` if self talked to another object.
    pub fn talked_to(&self, obj: Object) -> bool {
        self.matches(&Object::DidTo(Verb::Talk, Box::new(obj)))
    }
    /// Returns `true` if self was talked to by another object.
    pub fn was_talked_to_by(&self, obj: Object) -> bool {
        self.matches(&Object::WasBy(Verb::Talk, Box::new(obj)))
    }
    /// Returns `true` if self was moved by another object.
    pub fn was_moved_by(&self, obj: Object) -> bool {
        self.matches(&Object::WasBy(Verb::Move, Box::new(obj)))
    }
    /// Returns `true` if self moved another object.
    pub fn moved(&self, obj: Object) -> bool {
        self.matches(&Object::DidTo(Verb::Move, Box::new(obj)))
    }
    /// Returns `true` if self is opponent of another object.
    pub fn is_opponent_of(&self, obj: Object) -> bool {
        self.matches(&opponent_of(obj).into())
    }
    /// Returns `true` if self locked another object.
    pub fn locked(&self, obj: Object) -> bool {
        self.matches(&Object::DidTo(Verb::Lock, Box::new(obj)))
    }
    /// Returns `true` if self closed another object.
    pub fn closed(&self, obj: Object) -> bool {
        self.matches(&Object::DidTo(Verb::Close, Box::new(obj)))
    }
}
