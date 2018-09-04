//! # AdvancedResearch-Room: An experiment to test The Room Hypothesis of Common Sense
//!
//! Paper: [The Room Hypothesis of Common Sense](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/the-room-hypothesis-of-common-sense.pdf)
//!
//! The Room Hypothesis of Common Sense states that artificial common sense can be modeled
//! using extra constraints on predicates similar to those used in [Lojban](https://mw.lojban.org/papri/Lojban).
//!
//! These extra constraints assigns and uses sub-types on objects that the agent thinks about.
//! The "room" refers to a finite number of objects for which speech-acts can
//! determine whether common goals that the agent tries to achieve will fail.
//!
//! In the view of The Room Hypothesis, common sense is closely linked to [Zen Rationality](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/zen-rationality.pdf),
//! an extension of instrumental rationality with the ability for higher order reasoning about goals.
//!
//! With other words, common sense is a way for a zen rational agent to "factor out" common
//! terms in its utility function into a background theory of efficient higher order behavior.
//! This factorization leads to evolved concepts that corresponds to speech acts in natural language.
//!
//! This experiment is to test this hypothesis structurally, instead of using machine learning.
//! The motivation is to derive which kind of constraints that occur naturally,
//! such that these constraints can later be translated into machine learning problems.

use std::sync::Arc;

pub use verb::Verb;

mod verb;
mod actions;

#[derive(Clone, Debug, PartialEq)]
pub enum Adjective {
    Dead,
    Murderer,
    Open,
    Closed,
    Locked,
    Unlocked,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Placement {
    On(Object),
    LeanToward(Object),
    In(Object),
    OutOf(Object),
}

impl Placement {
    pub fn matches(&self, other: &Placement) -> bool {
        use Placement::*;

        match (self, other) {
            (&On(ref a), &On(ref b)) => a.matches(b),
            (&LeanToward(ref a), &LeanToward(ref b)) => a.matches(b),
            (&In(ref a), &In(ref b)) => a.matches(b),
            (&OutOf(ref a), &OutOf(ref b)) => a.matches(b),

            (&On(_), _) | (_, &On(_)) => false,
            (&LeanToward(_), _) | (_, &LeanToward(_)) => false,
            (&In(_), _) | (_, &In(_)) => false,
        }
    }

    pub fn obj_ref(&self) -> &Object {
        use Placement::*;

        match self {
            On(ref obj) => obj,
            LeanToward(ref obj) => obj,
            In(ref obj) => obj,
            OutOf(ref obj) => obj,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Role {
    OpponentOf(Object),
}

impl Role {
    pub fn matches(&self, other: &Role) -> bool {
        use Role::*;

        match (self, other) {
            (&OpponentOf(ref a), &OpponentOf(ref b)) => a.matches(b),
        }
    }

    pub fn obj_ref(&self) -> &Object {
        use Role::*;

        match self {
            OpponentOf(ref obj) => obj,
        }
    }
}

impl From<Adjective> for Object {
    fn from(adj: Adjective) -> Object {
        Object::Adj(adj)
    }
}

impl From<Placement> for Object {
    fn from(place: Placement) -> Object {
        Object::Placement(Box::new(place))
    }
}

impl From<Role> for Object {
    fn from(role: Role) -> Object {
        Object::Role(Box::new(role))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    I,
    You,
    He,
    She,
    It,
    That,
    And(Vec<Object>),
    Placement(Box<Placement>),
    Role(Box<Role>),
    Has(Box<Object>),
    HasNot(Box<Object>),
    Called(Arc<String>),
    OfType(Arc<String>),
    Adj(Adjective),
    WasBy(Verb, Box<Object>),
    DidTo(Verb, Box<Object>),
    KeyTo(Box<Object>),
}

impl Object {
    /// Returns `true` if the object matches another.
    pub fn matches(&self, other: &Object) -> bool {
        use Object::*;

        match (self, other) {
            (_, &And(ref objs)) => {
                // Match everyone since there are more than one criteria.
                objs.iter().all(|obj| self.matches(obj))
            }
            (&And(ref objs), _) => {
                objs.iter().any(|obj| obj.matches(other))
            }
            (&I, &I) => true,
            (&You, &You) => true,
            (&He, &He) => true,
            (&She, &She) => true,
            (&It, &It) => true,
            (&That, &That) => true,
            (&Placement(ref a), &Placement(ref b)) => a.matches(b),
            (&Role(ref a), &Role(ref b)) => a.matches(b),
            (&Has(ref a), &Has(ref b)) => a.matches(b),
            (&HasNot(ref a), &HasNot(ref b)) => a.matches(b),
            (&Called(ref a), &Called(ref b)) => a == b,
            (&OfType(ref a), &OfType(ref b)) => a == b,
            (&Adj(ref a), &Adj(ref b)) => a == b,
            (&WasBy(va, ref a), &WasBy(vb, ref b)) => va == vb && a.matches(b),
            (&DidTo(va, ref a), &DidTo(vb, ref b)) => va == vb && a.matches(b),
            (&KeyTo(ref a), &KeyTo(ref b)) => a.matches(b),

            (&I, _) | (_, &I) => false,
            (&You, _) | (_, &You) => false,
            (&He, _) | (_, &He) => false,
            (&She, _) | (_, &She) => false,
            (&It, _) | (_, &It) => false,
            (&That, _) | (_, &That) => false,
            (&Placement(_), _) | (_, &Placement(_)) => false,
            (&Role(_), _) | (_, &Role(_)) => false,
            (&Has(_), _) | (_, &Has(_)) => false,
            (&HasNot(_), _) | (_, &HasNot(_)) => false,
            (&Called(_), _) | (_, &Called(_)) => false,
            (&OfType(_), _) | (_, &OfType(_)) => false,
            (&Adj(_), _) | (_, &Adj(_)) => false,
            (&WasBy(_, _), _) | (_, &WasBy(_, _)) => false,
            (&DidTo(_, _), _) | (_, &DidTo(_, _)) => false,
        }
    }

    /// Adds object to list of properties.
    pub fn push(&mut self, obj: Object) {
        if let Object::And(ref mut list) = *self {
            if !list.iter().any(|o| o.matches(&obj)) {
                list.push(obj);
            }
        } else {
            if self.matches(&obj) {return};
            let copy = self.clone();
            *self = Object::And(vec![copy, obj]);
        }
    }

    /// Remove placement.
    pub fn remove_placement(&mut self) {
        use Object::*;

        if let And(ref mut list) = *self {
            for i in (0..list.len()).rev() {
                let found = if let Placement(_) = list[i] {true} else {false};
                if found {list.remove(i);}
            }
        }
    }

    /// Remove specific or any criteria.
    pub fn remove(&mut self, obj: &Object) {
        use Object::*;

        if let And(ref mut list) = *self {
            for i in (0..list.len()).rev() {
                if obj.matches(&list[i]) {
                    list.remove(i);
                }
            }
        }
    }

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
}

/// Stores an action.
pub enum Action {
    /// Do something.
    Do {
        /// The subject.
        subject: Object,
        /// The verb.
        verb: Verb,
        /// The object.
        object: Object,
        /// Decorate with new properties.
        decorate: Vec<(Object, Object)>,
        /// Remove properties.
        remove: Vec<(Object, Object)>,
        /// Remove placement properties.
        remove_placement: Vec<Object>,
        /// Properties that prevents the action from happening.
        prevent: Vec<(Object, Object)>,
        /// Required properties for the action to happen.
        require: Vec<(Object, Object)>,
        /// A group of distinct objects in order for the action to work at all.
        distinct: Vec<Object>,
    },
}

pub struct Room {
    pub objects: Vec<Object>
}

impl Room {
    /// Creates a new room with objects.
    pub fn new(objects: Vec<Object>) -> Room {Room {objects}}

    /// Finds object in room.
    ///
    /// Returns `Err` if there are more than one object that can be identified,
    /// or if there are no objects that could be identified.
    pub fn find(&self, obj: &Object) -> Result<usize, Vec<usize>> {
        let mut res = vec![];
        for i in 0..self.objects.len() {
            if self.objects[i].matches(obj) {
                res.push(i);
            }
        }
        if res.len() == 1 {Ok(res[0])}
        else {Err(res)}
    }

    /// Executate an action in the room.
    pub fn action(&mut self, action: &Action) -> Result<(), ()> {
        match *action {
            Action::Do {
                ref subject,
                verb,
                ref object,
                ref decorate,
                ref remove,
                ref remove_placement,
                ref require,
                ref prevent,
                ref distinct,
            } => {
                match (self.find(subject), self.find(object)) {
                    (Ok(a), Ok(b)) => {
                        // Check that objects in distinctive group are distinctive.
                        let mut ids = vec![];
                        for obj in distinct {
                            if let Ok(a) = self.find(obj) {
                                for i in 0..ids.len() {
                                    if ids[i] == a {return Err(())};
                                }
                                ids.push(a);
                            }
                        }

                        if !require.iter().all(|(ref obj, ref adj)| {
                            if let Ok(ind) = self.find(obj) {
                                self.objects[ind].matches(&adj.clone().into())
                            } else {
                                false
                            }
                        }) {return Err(())};
                        if prevent.iter().any(|&(ref obj, ref adj)| {
                            if let Ok(ind) = self.find(obj) {
                                self.objects[ind].matches(&adj.clone().into())
                            } else {
                                false
                            }
                        }) {return Err(())};
                        for &(ref obj, ref removal) in remove {
                            if let Ok(ind) = self.find(obj) {
                                self.objects[ind].remove(&removal.clone().into());
                            }
                        }
                        for obj in remove_placement {
                            if let Ok(ind) = self.find(obj) {
                                self.objects[ind].remove_placement();
                            }
                        }
                        for &(ref obj, ref decor) in decorate {
                            if let Ok(ind) = self.find(obj) {
                                self.objects[ind].push(decor.clone());
                            }
                        }
                        self.objects[a].push(Object::DidTo(verb, Box::new(object.clone())));
                        self.objects[b].push(Object::WasBy(verb, Box::new(subject.clone())));
                        Ok(())
                    }
                    _ => Err(())
                }
            }
        }
    }
}

pub fn on(obj: Object) -> Placement {Placement::On(obj)}
pub fn lean_toward(obj: Object) -> Placement {Placement::LeanToward(obj)}
pub fn in_(obj: Object) -> Placement {Placement::In(obj)}
pub fn out_of(obj: Object) -> Placement {Placement::OutOf(obj)}
pub fn has(obj: Object) -> Object {Object::Has(Box::new(obj))}
pub fn has_not(obj: Object) -> Object {Object::HasNot(Box::new(obj))}
pub fn called(name: &str) -> Object {Object::Called(Arc::new(name.into()))}
pub fn of_type(name: &str) -> Object {Object::OfType(Arc::new(name.into()))}
pub fn key_to(obj: Object) -> Object {Object::KeyTo(Box::new(obj))}

pub fn killed_by(obj: Object) -> Object {Object::WasBy(Verb::Kill, Box::new(obj))}
pub fn killed(obj: Object) -> Object {Object::DidTo(Verb::Kill, Box::new(obj))}

pub fn opponent_of(obj: Object) -> Role {Role::OpponentOf(obj)}

#[cfg(test)]
mod tests {
    use super::*;
    use Object::*;
    use Adjective::*;

    #[test]
    fn test_move() {
        let it = 1;
        let that = 2;
        let mut room = Room::new(vec![He, It, That]);
        room.action(&He.moves(It, on(That))).unwrap();
        assert!(room.objects[it].is_on(That));
        assert!(!room.objects[that].is_on(It));
        room.action(&He.moves(That, on(It))).unwrap();
        assert!(room.objects[that].is_on(It));
        assert!(!room.objects[it].is_on(That));

        let he = 0;
        let that = 1;
        let mut room = Room::new(vec![He, That]);
        // Can not move itself.
        assert!(room.action(&He.moves(He, on(That))).is_err());
        // Can move something on itself.
        assert!(room.action(&He.moves(That, on(He))).is_ok());
        assert!(room.objects[that].is_on(He));
        assert!(room.objects[that].was_moved_by(He));
        assert!(room.objects[he].moved(That));
    }

    #[test]
    fn test_give() {
        let he = 0;
        let she = 1;
        let mut room = Room::new(vec![He, She, It]);
        room.action(&He.gives_item(She, It)).unwrap();
        assert!(room.objects[she].has(It));
        assert!(!room.objects[he].has(It));
        room.action(&She.gives_item(He, It)).unwrap();
        assert!(room.objects[he].has(It));
        assert!(!room.objects[she].has(It));

        let she = 0;
        let peter = 1;
        let mut room = Room::new(vec![She, called("Peter"), It]);
        room.action(&She.gives_item(called("Peter"), It)).unwrap();
        assert!(room.objects[peter].has(It));
        assert!(!room.objects[she].has(It));
        room.action(&called("Peter").gives_item(She, It)).unwrap();
        assert!(room.objects[she].has(It));
        assert!(!room.objects[peter].has(It));

        let mut room = Room::new(vec![He, It]);
        // Can not give something to the same object.
        assert!(room.action(&He.gives_item(He, It)).is_err());
        // The same object that gives can not be given.
        assert!(room.action(&He.gives_item(It, He)).is_err());
    }

    #[test]
    fn test_names() {
        let he = 0;
        let mut room = Room::new(vec![He]);
        room.objects[he].push(called("Peter"));
        assert!(room.objects[he].matches(&And(vec![He, called("Peter")])));
    }

    #[test]
    fn test_kill() {
        let he = 0;
        let she = 1;
        let mut room = Room::new(vec![He, She]);
        assert!(!room.objects[she].matches(&Dead.into()));
        assert!(!room.objects[he].matches(&Murderer.into()));
        room.action(&He.kills(She)).unwrap();
        assert!(room.objects[she].matches(&Dead.into()));
        assert!(room.objects[he].matches(&Murderer.into()));
        assert!(room.objects[she].was_killed_by(He));
        assert!(room.objects[he].killed(She));

        let peter = 0;
        let john = 1;
        let sheila = 2;
        let mut room = Room::new(vec![called("Peter"), called("John"), called("Sheila")]);
        assert!(!room.objects[peter].matches(&Dead.into()));
        assert!(!room.objects[peter].killed(called("John")));
        assert!(!room.objects[peter].was_killed_by(called("Sheila")));
        assert!(!room.objects[john].matches(&Dead.into()));
        assert!(!room.objects[john].was_killed_by(called("Peter")));
        assert!(!room.objects[sheila].matches(&Dead.into()));
        assert!(!room.objects[sheila].killed(killed(called("John"))));
        room.action(&called("Peter").kills(called("John"))).unwrap();
        room.action(&called("Sheila").kills(killed(called("John")))).unwrap();
        assert!(room.objects[peter].matches(&Dead.into()));
        assert!(room.objects[peter].killed(called("John")));
        assert!(room.objects[peter].was_killed_by(called("Sheila")));
        assert!(room.objects[john].matches(&Dead.into()));
        assert!(room.objects[john].was_killed_by(called("Peter")));
        assert!(!room.objects[sheila].matches(&Dead.into()));
        assert!(room.objects[sheila].killed(killed(called("John"))));
    }

    #[test]
    fn test_talk() {
        let i = 0;
        let you = 1;
        let mut room = Room::new(vec![I, You]);
        assert!(!room.objects[i].talked_to(You));
        assert!(!room.objects[you].was_talked_to_by(I));
        room.action(&I.talk_to(You)).unwrap();
        assert!(room.objects[i].talked_to(You));
        assert!(room.objects[you].was_talked_to_by(I));
    }

    #[test]
    fn test_door() {
        let door = 0;
        let mut room = Room::new(vec![of_type("door"), I]);
        assert!(!room.objects[door].matches(&Open.into()));
        assert!(!room.objects[door].matches(&Closed.into()));
        room.action(&I.opens(of_type("door"))).unwrap();
        assert!(room.objects[door].matches(&Open.into()));
        assert!(!room.objects[door].matches(&Closed.into()));
        room.action(&I.closes(of_type("door"))).unwrap();
        assert!(room.objects[door].matches(&Closed.into()));
        assert!(!room.objects[door].matches(&Open.into()));
        // Can not walk through door because it is closed.
        assert!(room.action(&I.walks_through(of_type("door"))).is_err());
        room.action(&I.opens(of_type("door"))).unwrap();
        room.action(&I.walks_through(of_type("door"))).unwrap();

        let mut room = Room::new(vec![of_type("door"), I, key_to(of_type("door"))]);
        room.action(&I.closes(of_type("door"))).unwrap();
        room.action(&I.locks(of_type("door"))).unwrap();
        // Can not open door because it is locked.
        assert!(room.action(&I.opens(of_type("door"))).is_err());
        // Can not unlock door because I do not have the key.
        assert!(room.action(&I.unlocks(of_type("door"))).is_err());
        room.action(&I.picks_up(key_to(of_type("door")))).unwrap();
        room.action(&I.unlocks(of_type("door"))).unwrap();
        room.action(&I.opens(of_type("door"))).unwrap();

        let mut room = Room::new(vec![
            And(vec![He, has_not(key_to(of_type("door")))]),
            She,
            key_to(of_type("door"))
        ]);
        assert!(room.action(&He.gives_item(She, key_to(of_type("door")))).is_err());
        room.action(&He.picks_up(key_to(of_type("door")))).unwrap();
        room.action(&He.gives_item(She, key_to(of_type("door")))).unwrap();
    }

    #[test]
    fn test_ladder() {
        let ladder = 0;
        let i = 4;
        let mut room = Room::new(vec![
            of_type("ladder"),
            of_type("roof"),
            of_type("ground"),
            of_type("wall"),
            And(vec![I, has_not(of_type("ladder"))]),
        ]);
        room.action(&I.carries(of_type("ladder"))).unwrap();
        assert!(!room.objects[i].has_not(of_type("ladder")));
        assert!(room.objects[i].has(of_type("ladder")));
        room.action(&I.puts_down(of_type("ladder"))).unwrap();
        assert!(!room.objects[i].has(of_type("ladder")));
        assert!(room.objects[i].has_not(of_type("ladder")));
        room.action(&of_type("ladder").stands_on(of_type("ground"))).unwrap();
        assert!(room.objects[ladder].is_on(of_type("ground")));
        room.action(&of_type("ladder").leans_toward(of_type("wall"))).unwrap();
        assert!(room.objects[ladder].is_leaning_toward(of_type("wall")));
        room.action(&I.climbs_to(of_type("ladder"), on(of_type("roof")))).unwrap();
        assert!(room.objects[i].is_on(of_type("roof")));
        room.action(&I.picks_up(of_type("ladder"))).unwrap();
        assert!(!room.objects[ladder].is_on(of_type("ground")));
    }

    #[test]
    fn test_sleep() {
        let i = 1;
        let mut room = Room::new(vec![
            of_type("bed"),
            I
        ]);
        room.action(&I.sleeps_in(of_type("bed"))).unwrap();
        assert!(room.objects[i].is_in(of_type("bed")));
        room.action(&I.wakes_up_in(of_type("bed"))).unwrap();
        room.action(&I.climbs_out_of(of_type("bed"))).unwrap();
        assert!(!room.objects[i].is_in(of_type("bed")));
        assert!(room.objects[i].is_out_of(of_type("bed")));
        room.action(&I.climbs_into(of_type("bed"))).unwrap();
        assert!(!room.objects[i].is_out_of(of_type("bed")));
        assert!(room.objects[i].is_in(of_type("bed")));
    }

    #[test]
    fn test_chess() {
        let i = 1;
        let you = 2;
        let mut room = Room::new(vec![
            And(vec![called("chess"), of_type("game")]),
            I,
            You
        ]);
        room.action(&I.plays_against(called("chess"), You)).unwrap();
        assert!(room.objects[i].is_opponent_of(You));
        assert!(room.objects[you].is_opponent_of(I));
    }
}
