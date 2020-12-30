use spatialos_sys::*;

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

impl From<EntityIdConstraint> for Worker_EntityIdConstraint {
    fn from(constraint: EntityIdConstraint) -> Self {
        Self {
            entity_id: constraint.entity_id,
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

impl From<ComponentConstraint> for Worker_ComponentConstraint {
    fn from(constraint: ComponentConstraint) -> Self {
        Self {
            component_id: constraint.component_id,
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

impl From<SphereConstraint> for Worker_SphereConstraint {
    fn from(constraint: SphereConstraint) -> Self {
        Self {
            x: constraint.x,
            y: constraint.y,
            z: constraint.z,
            radius: constraint.radius,
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

impl From<OrConstraint> for Worker_OrConstraint {
    fn from(_constraint: OrConstraint) -> Self {
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

impl From<AndConstraint> for Worker_AndConstraint {
    fn from(_constraint: AndConstraint) -> Self {
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

impl From<NotConstraint> for Worker_NotConstraint {
    fn from(constraint: NotConstraint) -> Self {
        let constraint = *constraint.constraint;
        let constraint = Box::new(Worker_Constraint::from(constraint));
        let constraint = Box::into_raw(constraint);
        Self { constraint }
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

impl From<Constraint> for Worker_Constraint {
    fn from(constraint: Constraint) -> Self {
        match constraint {
            Constraint::EntityId(entity_id) => Self {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_ENTITY_ID.into(),
                constraint: Worker_Constraint_Union {
                    entity_id_constraint: entity_id.into(),
                },
            },
            Constraint::Component(component) => Self {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_COMPONENT.into(),
                constraint: Worker_Constraint_Union {
                    component_constraint: component.into(),
                },
            },
            Constraint::Sphere(sphere) => Self {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_SPHERE.into(),
                constraint: Worker_Constraint_Union {
                    sphere_constraint: sphere.into(),
                },
            },
            Constraint::And(and) => Self {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_AND.into(),
                constraint: Worker_Constraint_Union {
                    and_constraint: and.into(),
                },
            },
            Constraint::Or(or) => Self {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_OR.into(),
                constraint: Worker_Constraint_Union {
                    or_constraint: or.into(),
                },
            },
            Constraint::Not(not) => Self {
                constraint_type: Worker_ConstraintType::WORKER_CONSTRAINT_TYPE_NOT.into(),
                constraint: Worker_Constraint_Union {
                    not_constraint: not.into(),
                },
            },
        }
    }
}
