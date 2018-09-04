use *;

impl Object {
    /// Moves object to some place.
    pub fn moves(self, object: Object, place: Placement) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::Move, object: object.clone(),
            decorate: vec![(object.clone(), place.clone().into())],
            remove: if let Placement::On(_) = place {
                vec![(place.obj_ref().clone(), on(object.clone()).into())]
            } else {
                vec![]
            },
            remove_placement: vec![object.clone()],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, object],
        }
    }

    /// Give object an item.
    ///
    /// The item must be unique,
    /// such that the subject no longer has the object.
    pub fn gives_item(self, to: Object, item: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::Give, object: to.clone(),
            decorate: vec![
                (self.clone(), has_not(item.clone())),
                (to.clone(), has(item.clone())),
            ],
            remove: vec![
                (self.clone(), has(item.clone())),
                (to.clone(), has_not(item.clone())),
            ],
            remove_placement: vec![
                item.clone(),
            ],
            require: vec![],
            prevent: vec![
                (self.clone(), has_not(item.clone())),
            ],
            distinct: vec![self, to, item],
        }
    }

    /// Give object an object.
    ///
    /// The item must be unique,
    /// such that the subject no longer has the object.
    pub fn gives_to(self, item: Object, to: Object) -> Action {
        self.gives_item(to, item)
    }

    /// Kill object.
    pub fn kills(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::Kill, object: object.clone(),
            decorate: vec![
                (self, Adjective::Murderer.into()),
                (object, Adjective::Dead.into()),
            ],
            remove: vec![],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![],
            distinct: vec![],
        }
    }

    /// Talk to object.
    pub fn talk_to(self, object: Object) -> Action {
        Action::Do {
            subject: self, verb: Verb::Talk, object,
            decorate: vec![],
            remove: vec![],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![],
            distinct: vec![],
        }
    }

    /// Opens object.
    pub fn opens(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::Open, object: object.clone(),
            decorate: vec![
                (object.clone(), Adjective::Open.into())
            ],
            remove: vec![
                (object.clone(), Adjective::Closed.into())
            ],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![
                (object.clone(), Adjective::Locked.into()),
            ],
            distinct: vec![self, object],
        }
    }

    /// Closes object.
    pub fn closes(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::Close, object: object.clone(),
            decorate: vec![
                (object.clone(), Adjective::Closed.into())
            ],
            remove: vec![
                (object.clone(), Adjective::Open.into())
            ],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, object],
        }
    }

    /// Walks through.
    pub fn walks_through(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::WalkThrough, object: object.clone(),
            decorate: vec![],
            remove: vec![],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![
                (object.clone(), Adjective::Closed.into())
            ],
            distinct: vec![self, object],
        }
    }

    /// Locks object.
    pub fn locks(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::Lock, object: object.clone(),
            decorate: vec![
                (object.clone(), Adjective::Locked.into()),
                (object.clone(), Adjective::Closed.into()),
            ],
            remove: vec![
                (object.clone(), Adjective::Unlocked.into())
            ],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, object],
        }
    }

    /// Unlocks object.
    pub fn unlocks(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::Unlock, object: object.clone(),
            decorate: vec![
                (object.clone(), Adjective::Unlocked.into())
            ],
            remove: vec![
                (object.clone(), Adjective::Locked.into())
            ],
            remove_placement: vec![],
            require: vec![
                (self.clone(), has(key_to(object.clone()))),
            ],
            prevent: vec![],
            distinct: vec![self, object],
        }
    }

    /// Picks up object.
    pub fn picks_up(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::PickUp, object: object.clone(),
            decorate: vec![
                (self.clone(), has(object.clone()))
            ],
            remove: vec![
                (self.clone(), has_not(object.clone()))
            ],
            remove_placement: vec![
                object.clone(),
            ],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, object],
        }
    }

    /// Climbs object.
    pub fn climbs_to(self, object: Object, place: Placement) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::Climb, object: object.clone(),
            decorate: vec![
                (self.clone(), place.clone().into())
            ],
            remove: vec![],
            remove_placement: vec![
                self.clone()
            ],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, object, place.into()],
        }
    }

    /// Climbs out of object.
    pub fn climbs_out_of(self, object: Object) -> Action {
        self.climbs_to(object.clone(), out_of(object))
    }

    /// Climbs into object.
    pub fn climbs_into(self, object: Object) -> Action {
        self.climbs_to(object.clone(), in_(object))
    }

    /// Carries object.
    pub fn carries(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::Carry, object: object.clone(),
            decorate: vec![
                (self.clone(), has(object.clone()))
            ],
            remove: vec![
                (self.clone(), has_not(object.clone()))
            ],
            remove_placement: vec![
                object.clone(),
            ],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, object],
        }
    }

    /// Puts down object.
    pub fn puts_down(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::PutDown, object: object.clone(),
            decorate: vec![
                (self.clone(), has_not(object.clone()))
            ],
            remove: vec![
                (self.clone(), has(object.clone())),
            ],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, object],
        }
    }

    /// Stand on object.
    pub fn stands_on(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::StandOn, object: object.clone(),
            decorate: vec![
                (self.clone(), on(object.clone()).into())
            ],
            remove: vec![],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, object],
        }
    }

    /// Lean toward object.
    pub fn leans_toward(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::LeanToward, object: object.clone(),
            decorate: vec![
                (self.clone(), lean_toward(object.clone()).into())
            ],
            remove: vec![],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, object],
        }
    }

    /// Sleeps in object.
    pub fn sleeps_in(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::SleepIn, object: object.clone(),
            decorate: vec![
                (self.clone(), in_(object.clone()).into())
            ],
            remove: vec![],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, object],
        }
    }

    /// Wakes up in object.
    pub fn wakes_up_in(self, object: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::WakeUpIn, object: object.clone(),
            decorate: vec![
                (self.clone(), in_(object.clone()).into())
            ],
            remove: vec![],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, object],
        }
    }

    /// Play game against opponent.
    pub fn plays_against(self, game: Object, opponent: Object) -> Action {
        Action::Do {
            subject: self.clone(), verb: Verb::Play, object: game.clone(),
            decorate: vec![
                (self.clone(), opponent_of(opponent.clone()).into()),
                (opponent.clone(), opponent_of(self.clone()).into()),
            ],
            remove: vec![],
            remove_placement: vec![],
            require: vec![],
            prevent: vec![],
            distinct: vec![self, game, opponent],
        }
    }
}
