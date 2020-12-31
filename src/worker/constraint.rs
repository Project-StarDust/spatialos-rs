use spatialos_sys::{
    Worker_AndConstraint, Worker_ComponentConstraint, Worker_Constraint, Worker_ConstraintType,
    Worker_Constraint_Union, Worker_EntityIdConstraint, Worker_NotConstraint, Worker_OrConstraint,
    Worker_SphereConstraint,
};

use crate::worker::ComponentId;
use crate::worker::EntityId;

pub struct EntityIdConstraint {
    pub entity_id: EntityId,
}

impl From<Worker_EntityIdConstraint> for EntityIdConstraint {
    fn from(constraint: Worker_EntityIdConstraint) -> Self {
        Self {
            entity_id: constraint.entity_id,
        }
    }
}

impl Into<Worker_EntityIdConstraint> for EntityIdConstraint {
    fn into(self) -> Worker_EntityIdConstraint {
        Worker_EntityIdConstraint {
            entity_id: self.entity_id,
        }
    }
}

pub struct ComponentConstraint {
    pub component_id: ComponentId,
}

impl From<Worker_ComponentConstraint> for ComponentConstraint {
    fn from(constraint: Worker_ComponentConstraint) -> Self {
        Self {
            component_id: constraint.component_id,
        }
    }
}

impl Into<Worker_ComponentConstraint> for ComponentConstraint {
    fn into(self) -> Worker_ComponentConstraint {
        Worker_ComponentConstraint {
            component_id: self.component_id,
        }
    }
}

pub struct SphereConstraint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
}

impl From<Worker_SphereConstraint> for SphereConstraint {
    fn from(constraint: Worker_SphereConstraint) -> Self {
        Self {
            x: constraint.x,
            y: constraint.y,
            z: constraint.z,
            radius: constraint.radius,
        }
    }
}

impl Into<Worker_SphereConstraint> for SphereConstraint {
    fn into(self) -> Worker_SphereConstraint {
        Worker_SphereConstraint {
            x: self.x,
            y: self.y,
            z: self.z,
            radius: self.radius,
        }
    }
}

pub struct OrConstraint {
    pub constraints: Vec<Constraint>,
}

impl From<Worker_OrConstraint> for OrConstraint {
    fn from(constraint: Worker_OrConstraint) -> Self {
        let constraints = unsafe {
            let mut constraints = Vec::new();
            for index in 0..constraint.constraint_count {
                let constraint_ptr = constraint.constraints.offset(index as isize);
                constraints.push(Constraint::from(*constraint_ptr));
            }
            constraints
        };
        Self { constraints }
    }
}

impl Into<Worker_OrConstraint> for OrConstraint {
    fn into(self) -> Worker_OrConstraint {
        todo!()
    }
}

pub struct AndConstraint {
    pub constraints: Vec<Constraint>,
}

impl From<Worker_AndConstraint> for AndConstraint {
    fn from(constraint: Worker_AndConstraint) -> Self {
        let constraints = unsafe {
            let mut constraints = Vec::new();
            for index in 0..constraint.constraint_count {
                let constraint_ptr = constraint.constraints.offset(index as isize);
                constraints.push(Constraint::from(*constraint_ptr));
            }
            constraints
        };
        Self { constraints }
    }
}

impl Into<Worker_AndConstraint> for AndConstraint {
    fn into(self) -> Worker_AndConstraint {
        todo!()
    }
}

pub struct NotConstraint {
    pub constraint: Box<Constraint>,
}

impl From<Worker_NotConstraint> for NotConstraint {
    fn from(constraint: Worker_NotConstraint) -> Self {
        let constraint = unsafe { *constraint.constraint };
        Self {
            constraint: Box::new(Constraint::from(constraint)),
        }
    }
}

impl Into<Worker_NotConstraint> for NotConstraint {
    fn into(self) -> Worker_NotConstraint {
        let constraint = *self.constraint;
        let constraint = Box::new(constraint.into());
        let constraint = Box::into_raw(constraint);
        Worker_NotConstraint { constraint }
    }
}

pub enum Constraint {
    EntityId(EntityIdConstraint),
    Component(ComponentConstraint),
    Sphere(SphereConstraint),
    And(AndConstraint),
    Or(OrConstraint),
    Not(NotConstraint),
}

impl From<Worker_Constraint> for Constraint {
    fn from(constraint: Worker_Constraint) -> Self {
        match constraint.constraint_type.into() {
            Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_ENTITY_ID => {
                Self::EntityId(EntityIdConstraint::from(unsafe {
                    constraint.constraint.entity_id_constraint
                }))
            }
            Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_COMPONENT => {
                Self::Component(ComponentConstraint::from(unsafe {
                    constraint.constraint.component_constraint
                }))
            }
            Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_SPHERE => {
                Self::Sphere(SphereConstraint::from(unsafe {
                    constraint.constraint.sphere_constraint
                }))
            }
            Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_AND => {
                Self::And(AndConstraint::from(unsafe {
                    constraint.constraint.and_constraint
                }))
            }
            Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_OR => {
                Self::Or(OrConstraint::from(unsafe {
                    constraint.constraint.or_constraint
                }))
            }
            Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_NOT => {
                Self::Not(NotConstraint::from(unsafe {
                    constraint.constraint.not_constraint
                }))
            }
        }
    }
}

impl Into<Worker_Constraint> for Constraint {
    fn into(self) -> Worker_Constraint {
        match self {
            Self::EntityId(entity_id) => Worker_Constraint {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_ENTITY_ID.into(),
                constraint: Worker_Constraint_Union {
                    entity_id_constraint: entity_id.into(),
                },
            },
            Self::Component(component) => Worker_Constraint {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_COMPONENT.into(),
                constraint: Worker_Constraint_Union {
                    component_constraint: component.into(),
                },
            },
            Self::Sphere(sphere) => Worker_Constraint {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_SPHERE.into(),
                constraint: Worker_Constraint_Union {
                    sphere_constraint: sphere.into(),
                },
            },
            Self::And(and) => Worker_Constraint {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_AND.into(),
                constraint: Worker_Constraint_Union {
                    and_constraint: and.into(),
                },
            },
            Self::Or(or) => Worker_Constraint {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_OR.into(),
                constraint: Worker_Constraint_Union {
                    or_constraint: or.into(),
                },
            },
            Self::Not(not) => Worker_Constraint {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_NOT.into(),
                constraint: Worker_Constraint_Union {
                    not_constraint: not.into(),
                },
            },
        }
    }
}
