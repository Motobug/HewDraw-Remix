// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn header_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    let status_kind_prev = StatusModule::prev_status_kind(boma, 0);
    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && [*FIGHTER_STATUS_KIND_SPECIAL_S,*FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING].contains(&status_kind_prev)
        && situation_kind == *SITUATION_KIND_AIR {
        if  !VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL) {
            VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL);
            ControlModule::reset_trigger(boma);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
        }
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_BREATH,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_FAILURE
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    nspecial_cancels(boma, status_kind, situation_kind);
    header_cancel(boma, id, status_kind, situation_kind);
    fastfall_specials(fighter);

}

#[utils::macros::opff(FIGHTER_KIND_WIIFIT )]
pub fn wiifit_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		wiifit_frame(fighter)
    }
}

pub unsafe fn wiifit_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}