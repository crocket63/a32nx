#ifndef RTW_HEADER_A380PrimComputer_h_
#define RTW_HEADER_A380PrimComputer_h_
#include "rtwtypes.h"
#include "A380PrimComputer_types.h"
#include "A380LateralNormalLaw.h"
#include "A380LateralDirectLaw.h"
#include "A380PitchNormalLaw.h"
#include "A380PitchAlternateLaw.h"
#include "A380PitchDirectLaw.h"

extern const real_T A380PrimComputer_RGND;
extern const boolean_T A380PrimComputer_BGND;
extern base_prim_logic_outputs rtP_prim_logic_output_MATLABStruct;
extern base_prim_laws_outputs rtP_prim_laws_output_MATLABStruct;
extern base_prim_analog_outputs rtP_prim_analog_output_MATLABStruct;
extern base_prim_discrete_outputs rtP_prim_discrete_output_MATLABStruct;
class A380PrimComputer final
{
 public:
  struct rtDW_RateLimiter_A380PrimComputer_T {
    real_T pY;
    boolean_T pY_not_empty;
  };

  struct rtDW_RateLimiter_A380PrimComputer_g_T {
    real_T pY;
    boolean_T pY_not_empty;
  };

  struct rtDW_RateLimiter_A380PrimComputer_d_T {
    real_T pY;
    boolean_T pY_not_empty;
  };

  struct rtDW_LagFilter_A380PrimComputer_T {
    real_T pY;
    real_T pU;
    boolean_T pY_not_empty;
    boolean_T pU_not_empty;
  };

  struct rtDW_MATLABFunction_A380PrimComputer_j_T {
    boolean_T previousInput;
    boolean_T previousInput_not_empty;
  };

  struct rtDW_MATLABFunction_A380PrimComputer_k_T {
    real_T timeSinceCondition;
    boolean_T output;
  };

  struct rtDW_MATLABFunction_A380PrimComputer_km_T {
    boolean_T output;
  };

  struct rtDW_RateLimiter_A380PrimComputer_b_T {
    real_T pY;
    boolean_T pY_not_empty;
  };

  struct BlockIO_A380PrimComputer_T {
    base_prim_logic_outputs logic;
    base_prim_laws_outputs laws;
    real_T dt;
    real_T simulation_time;
    real_T capt_pitch_stick_pos;
    real_T fo_pitch_stick_pos;
    real_T capt_roll_stick_pos;
    real_T isis_1_bus;
    real_T isis_2_bus;
    real_T rate_gyro_pitch_1_bus;
    real_T rate_gyro_pitch_2_bus;
    real_T rate_gyro_roll_1_bus;
    real_T monotonic_time;
    real_T fo_roll_stick_pos;
    real_T rate_gyro_roll_2_bus;
    real_T rate_gyro_yaw_1_bus;
    real_T rate_gyro_yaw_2_bus;
    real_T speed_brake_lever_pos;
    real_T thr_lever_1_pos;
    real_T irdc_1_bus;
    real_T irdc_2_bus;
    real_T irdc_3_bus;
    real_T thr_lever_2_pos;
    real_T irdc_4_a_bus;
    real_T irdc_4_b_bus;
    real_T fcu_own_bus;
    real_T fcu_opp_bus;
    real_T thr_lever_3_pos;
    real_T thr_lever_4_pos;
    real_T elevator_1_pos_deg;
    real_T elevator_2_pos_deg;
    real_T elevator_3_pos_deg;
    real_T ths_pos_deg;
    real_T left_aileron_1_pos_deg;
    real_T left_aileron_2_pos_deg;
    real_T right_aileron_1_pos_deg;
    real_T right_aileron_2_pos_deg;
    real_T left_spoiler_pos_deg;
    real_T right_spoiler_pos_deg;
    real_T rudder_1_pos_deg;
    real_T rudder_2_pos_deg;
    real_T rudder_pedal_pos;
    real_T yellow_hyd_pressure_psi;
    real_T green_hyd_pressure_psi;
    real_T vert_acc_1_g;
    real_T vert_acc_2_g;
    real_T vert_acc_3_g;
    real_T lat_acc_1_g;
    real_T sec_1_bus;
    real_T sec_2_bus;
    real_T sec_3_bus;
    real_T lat_acc_2_g;
    real_T lat_acc_3_g;
    real_T left_body_wheel_speed;
    real_T left_wing_wheel_speed;
    real_T right_body_wheel_speed;
    real_T right_wing_wheel_speed;
    uint32_T SSM;
    uint32_T SSM_k;
    uint32_T SSM_kx;
    uint32_T SSM_kxx;
    uint32_T SSM_kxxt;
    uint32_T SSM_kxxta;
    uint32_T SSM_kxxtac;
    uint32_T SSM_kxxtac0;
    uint32_T SSM_kxxtac0z;
    uint32_T SSM_kxxtac0zt;
    uint32_T SSM_kxxtac0ztg;
    uint32_T SSM_kxxtac0ztgf;
    uint32_T SSM_kxxtac0ztgf2;
    uint32_T SSM_kxxtac0ztgf2u;
    uint32_T SSM_kxxtac0ztgf2ux;
    uint32_T SSM_kxxtac0ztgf2uxn;
    uint32_T SSM_ky;
    uint32_T SSM_d;
    uint32_T SSM_h;
    uint32_T SSM_kb;
    uint32_T SSM_p;
    uint32_T SSM_di;
    uint32_T SSM_j;
    uint32_T SSM_i;
    uint32_T SSM_g;
    uint32_T SSM_db;
    uint32_T SSM_n;
    uint32_T SSM_a;
    uint32_T SSM_ir;
    uint32_T SSM_hu;
    uint32_T SSM_e;
    uint32_T SSM_gr;
    uint32_T SSM_ev;
    uint32_T SSM_l;
    uint32_T SSM_ei;
    uint32_T SSM_an;
    uint32_T SSM_c;
    uint32_T SSM_cb;
    uint32_T SSM_lb;
    uint32_T SSM_ia;
    uint32_T SSM_kyz;
    uint32_T SSM_as;
    uint32_T SSM_is;
    uint32_T SSM_ca;
    uint32_T SSM_o;
    uint32_T SSM_ak;
    uint32_T SSM_cbj;
    uint32_T SSM_cu;
    uint32_T SSM_nn;
    uint32_T SSM_b;
    uint32_T SSM_m;
    uint32_T SSM_f;
    uint32_T SSM_bp;
    uint32_T SSM_hb;
    uint32_T SSM_gz;
    uint32_T SSM_pv;
    uint32_T SSM_mf;
    uint32_T SSM_m0;
    uint32_T SSM_kd;
    uint32_T SSM_pu;
    uint32_T SSM_nv;
    uint32_T SSM_d5;
    uint32_T SSM_eo;
    uint32_T SSM_nd;
    uint32_T SSM_bq;
    uint32_T SSM_hi;
    uint32_T SSM_mm;
    uint32_T SSM_kz;
    uint32_T SSM_il;
    uint32_T SSM_i2;
    uint32_T SSM_ah;
    uint32_T SSM_en;
    uint32_T SSM_dq;
    uint32_T SSM_px;
    uint32_T SSM_lbo;
    uint32_T SSM_p5;
    uint32_T SSM_mk;
    uint32_T SSM_mu;
    uint32_T SSM_cbl;
    uint32_T SSM_gzd;
    uint32_T SSM_mo;
    uint32_T SSM_me;
    uint32_T SSM_mj;
    uint32_T SSM_a5;
    uint32_T SSM_bt;
    uint32_T SSM_om;
    uint32_T SSM_ar;
    uint32_T SSM_ce;
    uint32_T SSM_ed;
    uint32_T SSM_jh;
    uint32_T SSM_je;
    uint32_T SSM_jt;
    uint32_T SSM_cui;
    uint32_T SSM_mq;
    uint32_T SSM_ni;
    uint32_T SSM_df;
    uint32_T SSM_oe;
    uint32_T SSM_ha;
    uint32_T SSM_op;
    uint32_T SSM_a50;
    uint32_T SSM_og;
    uint32_T SSM_a4;
    uint32_T SSM_bv;
    uint32_T SSM_bo;
    uint32_T SSM_d1;
    uint32_T SSM_hy;
    uint32_T SSM_gi;
    uint32_T SSM_pp;
    uint32_T SSM_iab;
    uint32_T SSM_jtv;
    uint32_T SSM_fy;
    uint32_T SSM_d4;
    uint32_T SSM_ars;
    uint32_T SSM_din;
    uint32_T SSM_m3;
    uint32_T SSM_np;
    uint32_T SSM_ax;
    uint32_T SSM_cl;
    uint32_T SSM_es;
    uint32_T SSM_gi1;
    uint32_T SSM_jz;
    uint32_T SSM_kt;
    uint32_T SSM_ds;
    uint32_T SSM_eg;
    uint32_T SSM_a0;
    uint32_T SSM_cv;
    uint32_T SSM_ea;
    uint32_T SSM_p4;
    uint32_T SSM_m2;
    uint32_T SSM_bt0;
    uint32_T SSM_nr;
    uint32_T SSM_fr;
    uint32_T SSM_cc;
    uint32_T SSM_lm;
    uint32_T SSM_mkm;
    uint32_T SSM_jhd;
    uint32_T SSM_av;
    uint32_T SSM_ira;
    uint32_T SSM_ge;
    uint32_T SSM_lv;
    uint32_T SSM_cg;
    uint32_T SSM_be;
    uint32_T SSM_axb;
    uint32_T SSM_nz;
    uint32_T SSM_cx;
    uint32_T SSM_gh;
    uint32_T SSM_ks;
    uint32_T SSM_pw;
    uint32_T SSM_fh;
    uint32_T SSM_gzn;
    uint32_T SSM_oo;
    uint32_T SSM_evh;
    uint32_T SSM_cn;
    uint32_T SSM_co;
    uint32_T SSM_pe;
    uint32_T SSM_cgz;
    uint32_T SSM_fw;
    uint32_T SSM_h4;
    uint32_T SSM_cb3;
    uint32_T SSM_pj;
    uint32_T SSM_dv;
    uint32_T SSM_i4;
    uint32_T SSM_fm;
    uint32_T SSM_e5;
    uint32_T SSM_bf;
    uint32_T SSM_fd;
    uint32_T SSM_fv;
    uint32_T SSM_dt;
    uint32_T SSM_j5;
    uint32_T SSM_ng;
    uint32_T SSM_cs;
    uint32_T SSM_ls;
    uint32_T SSM_dg;
    uint32_T SSM_d3;
    uint32_T SSM_p2;
    uint32_T SSM_bo0;
    uint32_T SSM_bc;
    uint32_T SSM_h0;
    uint32_T SSM_giz;
    uint32_T SSM_mqp;
    uint32_T SSM_ba;
    uint32_T SSM_in;
    uint32_T SSM_ff;
    uint32_T SSM_ic;
    uint32_T SSM_fs;
    uint32_T SSM_ja;
    uint32_T SSM_js;
    uint32_T SSM_is3;
    uint32_T SSM_ag;
    uint32_T SSM_f5;
    uint32_T SSM_ph;
    uint32_T SSM_jw;
    uint32_T SSM_jy;
    uint32_T SSM_j1;
    uint32_T SSM_ov;
    uint32_T SSM_mx;
    uint32_T SSM_b4;
    uint32_T SSM_gb;
    uint32_T SSM_oh;
    uint32_T SSM_mm5;
    uint32_T SSM_br;
    uint32_T SSM_c2;
    uint32_T SSM_hc;
    uint32_T SSM_ktr;
    uint32_T SSM_gl;
    uint32_T SSM_my;
    uint32_T SSM_j3;
    uint32_T SSM_go;
    uint32_T SSM_e5c;
    uint32_T SSM_dk;
    uint32_T SSM_evc;
    uint32_T SSM_kk;
    uint32_T SSM_af;
    uint32_T SSM_npr;
    uint32_T SSM_ew;
    uint32_T SSM_lt;
    uint32_T SSM_ger;
    uint32_T SSM_pxo;
    uint32_T SSM_co2;
    uint32_T SSM_ny;
    uint32_T SSM_l4;
    uint32_T SSM_nm;
    uint32_T SSM_nh;
    uint32_T SSM_dl;
    uint32_T SSM_dx;
    uint32_T SSM_a5h;
    uint32_T SSM_fl;
    uint32_T SSM_p3;
    uint32_T SSM_ns;
    uint32_T SSM_bm;
    uint32_T SSM_nl;
    uint32_T SSM_grm;
    uint32_T SSM_gzm;
    real32_T Data;
    real32_T Data_f;
    real32_T Data_fw;
    real32_T Data_fwx;
    real32_T Data_fwxk;
    real32_T Data_fwxkf;
    real32_T Data_fwxkft;
    real32_T Data_fwxkftc;
    real32_T Data_fwxkftc3;
    real32_T Data_fwxkftc3e;
    real32_T Data_fwxkftc3ep;
    real32_T Data_fwxkftc3epg;
    real32_T Data_fwxkftc3epgt;
    real32_T Data_fwxkftc3epgtd;
    real32_T Data_fwxkftc3epgtdx;
    real32_T Data_fwxkftc3epgtdxc;
    real32_T Data_h;
    real32_T Data_e;
    real32_T Data_j;
    real32_T Data_d;
    real32_T Data_p;
    real32_T Data_i;
    real32_T Data_g;
    real32_T Data_a;
    real32_T Data_eb;
    real32_T Data_jo;
    real32_T Data_ex;
    real32_T Data_fd;
    real32_T Data_ja;
    real32_T Data_k;
    real32_T Data_joy;
    real32_T Data_h3;
    real32_T Data_a0;
    real32_T Data_b;
    real32_T Data_eq;
    real32_T Data_iz;
    real32_T Data_j2;
    real32_T Data_o;
    real32_T Data_m;
    real32_T Data_oq;
    real32_T Data_fo;
    real32_T Data_p1;
    real32_T Data_p1y;
    real32_T Data_l;
    real32_T Data_kp;
    real32_T Data_k0;
    real32_T Data_pi;
    real32_T Data_dm;
    real32_T Data_f5;
    real32_T Data_js;
    real32_T Data_ee;
    real32_T Data_ig;
    real32_T Data_mk;
    real32_T Data_pu;
    real32_T Data_ly;
    real32_T Data_jq;
    real32_T Data_o5;
    real32_T Data_lyw;
    real32_T Data_gq;
    real32_T Data_n;
    real32_T Data_bq;
    real32_T Data_dmn;
    real32_T Data_jn;
    real32_T Data_c;
    real32_T Data_lx;
    real32_T Data_jb;
    real32_T Data_fn;
    real32_T Data_od;
    real32_T Data_ez;
    real32_T Data_pw;
    real32_T Data_m2;
    real32_T Data_ek;
    real32_T Data_iy;
    real32_T Data_lk;
    real32_T Data_ca;
    real32_T Data_pix;
    real32_T Data_di;
    real32_T Data_lz;
    real32_T Data_lu;
    real32_T Data_dc;
    real32_T Data_gc;
    real32_T Data_am;
    real32_T Data_mo;
    real32_T Data_dg;
    real32_T Data_e1;
    real32_T Data_fp;
    real32_T Data_ns;
    real32_T Data_m3;
    real32_T Data_oj;
    real32_T Data_jy;
    real32_T Data_j1;
    real32_T Data_fc;
    real32_T Data_of;
    real32_T Data_lg;
    real32_T Data_n4;
    real32_T Data_ot;
    real32_T Data_gv;
    real32_T Data_ou;
    real32_T Data_dh;
    real32_T Data_ph;
    real32_T Data_gs;
    real32_T Data_fd4;
    real32_T Data_hm;
    real32_T Data_i2;
    real32_T Data_og;
    real32_T Data_fv;
    real32_T Data_oc;
    real32_T Data_kq;
    real32_T Data_ne;
    real32_T Data_it;
    real32_T Data_ch;
    real32_T Data_bb;
    real32_T Data_ol;
    real32_T Data_hw;
    real32_T Data_hs;
    real32_T Data_fj;
    real32_T Data_ky;
    real32_T Data_h5;
    real32_T Data_ku;
    real32_T Data_jp;
    real32_T Data_nu;
    real32_T Data_br;
    real32_T Data_ae;
    real32_T Data_pe;
    real32_T Data_fy;
    real32_T Data_na;
    real32_T Data_my;
    real32_T Data_i4;
    real32_T Data_cx;
    real32_T Data_nz;
    real32_T Data_id;
    real32_T Data_o2;
    real32_T Data_gqq;
    real32_T Data_md;
    real32_T Data_cz;
    real32_T Data_pm;
    real32_T Data_bj;
    real32_T Data_ox;
    real32_T Data_pe5;
    real32_T Data_jj;
    real32_T Data_p5;
    real32_T Data_ekl;
    real32_T Data_nd;
    real32_T Data_n2;
    real32_T Data_dl;
    real32_T Data_gs2;
    real32_T Data_h4;
    real32_T Data_e3;
    real32_T Data_f5h;
    real32_T Data_an;
    real32_T Data_i4o;
    real32_T Data_af;
    real32_T Data_bm;
    real32_T Data_dk;
    real32_T Data_nv;
    real32_T Data_jpf;
    real32_T Data_i5;
    real32_T Data_k2;
    real32_T Data_as;
    real32_T Data_gk;
    real32_T Data_jl;
    real32_T Data_e32;
    real32_T Data_ih;
    real32_T Data_du;
    real32_T Data_nx;
    real32_T Data_n0;
    real32_T Data_eqi;
    real32_T Data_om;
    real32_T Data_nr;
    real32_T Data_p3;
    real32_T Data_nb;
    real32_T Data_hd;
    real32_T Data_al;
    real32_T Data_gu;
    real32_T Data_ix;
    real32_T Data_do;
    real32_T Data_hu;
    real32_T Data_pm1;
    real32_T Data_i2y;
    real32_T Data_pg;
    real32_T Data_ni;
    real32_T Data_fr;
    real32_T Data_cn;
    real32_T Data_nxl;
    real32_T Data_jh;
    real32_T Data_gl;
    real32_T Data_gn;
    real32_T Data_myb;
    real32_T Data_l2;
    real32_T Data_o5o;
    real32_T Data_l5;
    real32_T Data_dc2;
    real32_T Data_gr;
    real32_T Data_gp;
    real32_T Data_i3;
    real32_T Data_et;
    real32_T Data_mc;
    real32_T Data_k3;
    real32_T Data_f2;
    real32_T Data_gh;
    real32_T Data_ed;
    real32_T Data_o2j;
    real32_T Data_i43;
    real32_T Data_ic;
    real32_T Data_ak;
    real32_T Data_jg;
    real32_T Data_cu;
    real32_T Data_ep;
    real32_T Data_d3;
    real32_T Data_bt;
    real32_T Data_e0;
    real32_T Data_jl3;
    real32_T Data_nm;
    real32_T Data_ia;
    real32_T Data_j0;
    real32_T Data_bs;
    real32_T Data_hp;
    real32_T Data_ct;
    real32_T Data_pc;
    real32_T Data_nzt;
    real32_T Data_c0;
    real32_T Data_ojg;
    real32_T Data_lm;
    real32_T Data_fz;
    real32_T Data_oz;
    real32_T Data_gf;
    real32_T Data_nn;
    real32_T Data_a0z;
    real32_T Data_fk;
    real32_T Data_bu;
    real32_T Data_o23;
    real32_T Data_g3;
    real32_T Data_icc;
    boolean_T is_unit_1;
    boolean_T is_unit_2;
    boolean_T is_unit_3;
    boolean_T capt_priority_takeover_pressed;
    boolean_T fo_priority_takeover_pressed;
    boolean_T ap_1_puhsbutton_pressed;
    boolean_T ap_2_puhsbutton_pressed;
    boolean_T fcu_healthy;
    boolean_T athr_pushbutton;
    boolean_T ir_3_on_capt;
    boolean_T ir_3_on_fo;
    boolean_T adr_3_on_capt;
    boolean_T adr_3_on_fo;
    boolean_T pitch_trim_up_pressed;
    boolean_T pitch_trim_down_pressed;
    boolean_T green_low_pressure;
    boolean_T yellow_low_pressure;
    boolean_T slew_on;
    boolean_T pause_on;
    boolean_T tracking_mode_on_override;
    boolean_T tailstrike_protection_on;
    boolean_T computer_running;
    boolean_T prim_overhead_button_pressed;
  };

  struct D_Work_A380PrimComputer_T {
    real_T Delay_DSTATE;
    real_T Delay_DSTATE_c;
    real_T configFullEventTime;
    real_T eventTime;
    real_T resetEventTime;
    real_T eventTime_g;
    boolean_T Delay_DSTATE_cc;
    boolean_T Delay1_DSTATE;
    boolean_T Delay1_DSTATE_b;
    boolean_T Delay_DSTATE_f;
    uint8_T is_active_c28_A380PrimComputer;
    uint8_T is_c28_A380PrimComputer;
    boolean_T Memory_PreviousInput;
    boolean_T Memory_PreviousInput_j;
    boolean_T icLoad;
    boolean_T pLeftStickDisabled;
    boolean_T pRightStickDisabled;
    boolean_T configFullEventTime_not_empty;
    boolean_T ra1CoherenceRejected;
    boolean_T ra2CoherenceRejected;
    boolean_T eventTime_not_empty;
    boolean_T resetEventTime_not_empty;
    boolean_T sProtActive;
    boolean_T eventTime_not_empty_a;
    boolean_T abnormalConditionWasActive;
    boolean_T Runtime_MODE;
    rtDW_MATLABFunction_A380PrimComputer_k_T sf_MATLABFunction_al;
    rtDW_MATLABFunction_A380PrimComputer_j_T sf_MATLABFunction_nu;
    rtDW_MATLABFunction_A380PrimComputer_j_T sf_MATLABFunction_g4;
    rtDW_MATLABFunction_A380PrimComputer_k_T sf_MATLABFunction_j2;
    rtDW_MATLABFunction_A380PrimComputer_k_T sf_MATLABFunction_g24;
    rtDW_MATLABFunction_A380PrimComputer_k_T sf_MATLABFunction_lf;
    rtDW_MATLABFunction_A380PrimComputer_k_T sf_MATLABFunction_jl;
    rtDW_MATLABFunction_A380PrimComputer_k_T sf_MATLABFunction_jz;
    rtDW_RateLimiter_A380PrimComputer_b_T sf_RateLimiter_mr;
    rtDW_RateLimiter_A380PrimComputer_b_T sf_RateLimiter_ne;
    rtDW_MATLABFunction_A380PrimComputer_k_T sf_MATLABFunction_nb;
    rtDW_MATLABFunction_A380PrimComputer_km_T sf_MATLABFunction_br;
    rtDW_MATLABFunction_A380PrimComputer_km_T sf_MATLABFunction_jg;
    rtDW_MATLABFunction_A380PrimComputer_k_T sf_MATLABFunction_gfx;
    rtDW_MATLABFunction_A380PrimComputer_k_T sf_MATLABFunction_cj;
    rtDW_MATLABFunction_A380PrimComputer_j_T sf_MATLABFunction_jj;
    rtDW_MATLABFunction_A380PrimComputer_j_T sf_MATLABFunction_ej;
    rtDW_MATLABFunction_A380PrimComputer_j_T sf_MATLABFunction_ja;
    rtDW_MATLABFunction_A380PrimComputer_j_T sf_MATLABFunction_mb;
    rtDW_LagFilter_A380PrimComputer_T sf_LagFilter_a;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_ph;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_cda;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_p;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_cr;
    rtDW_LagFilter_A380PrimComputer_T sf_LagFilter;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_cd;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_j4;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_iy;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_np;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_lm;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_mn;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_au;
    rtDW_RateLimiter_A380PrimComputer_d_T sf_RateLimiter_md;
    rtDW_RateLimiter_A380PrimComputer_d_T sf_RateLimiter_me;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_f1;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_i;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_nl;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_gm;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_f;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_j;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_n2;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_bo;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_k;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_nh;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_gr;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_m;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_c5;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_d;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_l;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_n;
    rtDW_RateLimiter_A380PrimComputer_g_T sf_RateLimiter_a;
    rtDW_RateLimiter_A380PrimComputer_T sf_RateLimiter_g;
    rtDW_RateLimiter_A380PrimComputer_T sf_RateLimiter_c;
    rtDW_RateLimiter_A380PrimComputer_T sf_RateLimiter_b;
    rtDW_RateLimiter_A380PrimComputer_T sf_RateLimiter;
  };

  struct ExternalInputs_A380PrimComputer_T {
    prim_inputs in;
  };

  struct ExternalOutputs_A380PrimComputer_T {
    prim_outputs out;
  };

  struct Parameters_A380PrimComputer_T {
    real_T LagFilter_C1;
    real_T LagFilter_C1_e;
    real_T DiscreteDerivativeVariableTs_Gain;
    real_T DiscreteTimeIntegratorVariableTsLimit_Gain;
    real_T RateLimiterVariableTs2_InitialCondition;
    real_T RateLimiterVariableTs3_InitialCondition;
    real_T RateLimiterVariableTs4_InitialCondition;
    real_T RateLimiterVariableTs6_InitialCondition;
    real_T RateLimiterGenericVariableTs24_InitialCondition;
    real_T RateLimiterGenericVariableTs25_InitialCondition;
    real_T DiscreteDerivativeVariableTs_InitialCondition;
    real_T BitfromLabel_bit;
    real_T BitfromLabel1_bit;
    real_T BitfromLabel2_bit;
    real_T BitfromLabel1_bit_b;
    real_T BitfromLabel1_bit_n;
    real_T BitfromLabel3_bit;
    real_T BitfromLabel2_bit_l;
    real_T BitfromLabel1_bit_bv;
    real_T BitfromLabel1_bit_k;
    real_T BitfromLabel2_bit_e;
    real_T BitfromLabel3_bit_c;
    real_T BitfromLabel4_bit;
    real_T BitfromLabel5_bit;
    real_T BitfromLabel7_bit;
    real_T BitfromLabel_bit_i;
    real_T BitfromLabel1_bit_c;
    real_T BitfromLabel2_bit_p;
    real_T BitfromLabel3_bit_n;
    real_T BitfromLabel4_bit_j;
    real_T BitfromLabel5_bit_i;
    real_T BitfromLabel6_bit;
    real_T BitfromLabel7_bit_n;
    real_T BitfromLabel8_bit;
    real_T BitfromLabel9_bit;
    real_T BitfromLabel10_bit;
    real_T BitfromLabel11_bit;
    real_T BitfromLabel14_bit;
    real_T BitfromLabel15_bit;
    real_T BitfromLabel12_bit;
    real_T BitfromLabel13_bit;
    real_T BitfromLabel16_bit;
    real_T BitfromLabel17_bit;
    real_T BitfromLabel18_bit;
    real_T BitfromLabel19_bit;
    real_T BitfromLabel20_bit;
    real_T BitfromLabel21_bit;
    real_T BitfromLabel22_bit;
    real_T BitfromLabel23_bit;
    real_T BitfromLabel38_bit;
    real_T BitfromLabel39_bit;
    real_T BitfromLabel32_bit;
    real_T BitfromLabel33_bit;
    real_T BitfromLabel36_bit;
    real_T BitfromLabel37_bit;
    real_T BitfromLabel_bit_o;
    real_T BitfromLabel1_bit_e;
    real_T BitfromLabel2_bit_h;
    real_T BitfromLabel6_bit_k;
    real_T BitfromLabel3_bit_l;
    real_T BitfromLabel4_bit_b;
    real_T BitfromLabel5_bit_p;
    real_T BitfromLabel7_bit_h;
    real_T BitfromLabel_bit_h;
    real_T BitfromLabel1_bit_g;
    real_T BitfromLabel2_bit_n;
    real_T BitfromLabel3_bit_g;
    real_T BitfromLabel4_bit_e;
    real_T BitfromLabel5_bit_a;
    real_T BitfromLabel_bit_e;
    real_T BitfromLabel1_bit_d;
    real_T BitfromLabel_bit_l;
    real_T BitfromLabel_bit_p;
    real_T BitfromLabel1_bit_h;
    real_T BitfromLabel2_bit_f;
    real_T BitfromLabel3_bit_cv;
    real_T BitfromLabel4_bit_n;
    real_T BitfromLabel5_bit_py;
    real_T BitfromLabel_bit_n;
    real_T BitfromLabel1_bit_h1;
    real_T BitfromLabel2_bit_g;
    real_T BitfromLabel3_bit_b;
    real_T BitfromLabel4_bit_i;
    real_T BitfromLabel5_bit_l;
    real_T BitfromLabel_bit_of;
    real_T CompareToConstant_const;
    real_T CompareToConstant_const_l;
    real_T CompareToConstant3_const;
    real_T CompareToConstant4_const;
    real_T CompareToConstant1_const;
    real_T CompareToConstant2_const;
    real_T CompareToConstant_const_n;
    real_T CompareToConstant23_const;
    real_T CompareToConstant21_const;
    real_T CompareToConstant22_const;
    real_T CompareToConstant24_const;
    real_T CompareToConstant5_const;
    real_T CompareToConstant6_const;
    real_T CompareToConstant19_const;
    real_T CompareToConstant20_const;
    real_T CompareToConstant_const_m;
    real_T CompareToConstant15_const;
    real_T CompareToConstant1_const_p;
    real_T CompareToConstant2_const_i;
    real_T CompareToConstant3_const_e;
    real_T CompareToConstant4_const_c;
    real_T CompareToConstant13_const;
    real_T CompareToConstant14_const;
    real_T CompareToConstant10_const;
    real_T CompareToConstant7_const;
    real_T CompareToConstant16_const;
    real_T CompareToConstant17_const;
    real_T CompareToConstant18_const;
    real_T CompareToConstant8_const;
    real_T CompareToConstant9_const;
    real_T CompareToConstant_const_m4;
    real_T CompareToConstant_const_f;
    real_T CompareToConstant2_const_f;
    real_T CompareToConstant3_const_o;
    real_T CompareToConstant4_const_o;
    real_T CompareToConstant5_const_b;
    real_T CompareToConstant1_const_pv;
    real_T CompareToConstant1_const_d;
    real_T HysteresisNode2_highTrigger;
    real_T HysteresisNode3_highTrigger;
    real_T RateLimiterGenericVariableTs_lo;
    real_T RateLimiterGenericVariableTs1_lo;
    real_T RateLimiterVariableTs2_lo;
    real_T RateLimiterVariableTs3_lo;
    real_T RateLimiterGenericVariableTs_lo_k;
    real_T RateLimiterGenericVariableTs1_lo_i;
    real_T RateLimiterGenericVariableTs2_lo;
    real_T RateLimiterGenericVariableTs3_lo;
    real_T RateLimiterGenericVariableTs4_lo;
    real_T RateLimiterGenericVariableTs5_lo;
    real_T RateLimiterVariableTs4_lo;
    real_T RateLimiterVariableTs6_lo;
    real_T RateLimiterGenericVariableTs24_lo;
    real_T RateLimiterGenericVariableTs25_lo;
    real_T RateLimiterGenericVariableTs8_lo;
    real_T RateLimiterGenericVariableTs9_lo;
    real_T RateLimiterGenericVariableTs10_lo;
    real_T RateLimiterGenericVariableTs11_lo;
    real_T RateLimiterGenericVariableTs14_lo;
    real_T RateLimiterGenericVariableTs15_lo;
    real_T RateLimiterGenericVariableTs12_lo;
    real_T RateLimiterGenericVariableTs13_lo;
    real_T RateLimiterGenericVariableTs18_lo;
    real_T RateLimiterGenericVariableTs19_lo;
    real_T RateLimiterGenericVariableTs16_lo;
    real_T RateLimiterGenericVariableTs17_lo;
    real_T RateLimiterGenericVariableTs22_lo;
    real_T RateLimiterGenericVariableTs23_lo;
    real_T RateLimiterGenericVariableTs20_lo;
    real_T RateLimiterGenericVariableTs21_lo;
    real_T RateLimiterGenericVariableTs6_lo;
    real_T RateLimiterGenericVariableTs7_lo;
    real_T RateLimiterGenericVariableTs_lo_f;
    real_T RateLimiterGenericVariableTs1_lo_c;
    real_T RateLimiterGenericVariableTs2_lo_k;
    real_T RateLimiterGenericVariableTs3_lo_k;
    real_T HysteresisNode2_lowTrigger;
    real_T HysteresisNode3_lowTrigger;
    real_T ConfirmNode_timeDelay;
    real_T ConfirmNode2_timeDelay;
    real_T ConfirmNode1_timeDelay;
    real_T ConfirmNode_timeDelay_n;
    real_T ConfirmNode2_timeDelay_k;
    real_T ConfirmNode_timeDelay_d;
    real_T ConfirmNode1_timeDelay_a;
    real_T ConfirmNode_timeDelay_a;
    real_T ConfirmNode_timeDelay_g;
    real_T RateLimiterGenericVariableTs_up;
    real_T RateLimiterGenericVariableTs1_up;
    real_T RateLimiterVariableTs2_up;
    real_T RateLimiterVariableTs3_up;
    real_T RateLimiterGenericVariableTs_up_b;
    real_T RateLimiterGenericVariableTs1_up_k;
    real_T RateLimiterGenericVariableTs2_up;
    real_T RateLimiterGenericVariableTs3_up;
    real_T RateLimiterGenericVariableTs4_up;
    real_T RateLimiterGenericVariableTs5_up;
    real_T RateLimiterVariableTs4_up;
    real_T RateLimiterVariableTs6_up;
    real_T RateLimiterGenericVariableTs24_up;
    real_T RateLimiterGenericVariableTs25_up;
    real_T RateLimiterGenericVariableTs8_up;
    real_T RateLimiterGenericVariableTs9_up;
    real_T RateLimiterGenericVariableTs10_up;
    real_T RateLimiterGenericVariableTs11_up;
    real_T RateLimiterGenericVariableTs14_up;
    real_T RateLimiterGenericVariableTs15_up;
    real_T RateLimiterGenericVariableTs12_up;
    real_T RateLimiterGenericVariableTs13_up;
    real_T RateLimiterGenericVariableTs18_up;
    real_T RateLimiterGenericVariableTs19_up;
    real_T RateLimiterGenericVariableTs16_up;
    real_T RateLimiterGenericVariableTs17_up;
    real_T RateLimiterGenericVariableTs22_up;
    real_T RateLimiterGenericVariableTs23_up;
    real_T RateLimiterGenericVariableTs20_up;
    real_T RateLimiterGenericVariableTs21_up;
    real_T RateLimiterGenericVariableTs6_up;
    real_T RateLimiterGenericVariableTs7_up;
    real_T RateLimiterGenericVariableTs_up_a;
    real_T RateLimiterGenericVariableTs1_up_a;
    real_T RateLimiterGenericVariableTs2_up_l;
    real_T RateLimiterGenericVariableTs3_up_j;
    SignStatusMatrix EnumeratedConstant_Value;
    SignStatusMatrix EnumeratedConstant1_Value;
    a380_pitch_efcs_law EnumeratedConstant_Value_b;
    real32_T CompareToConstant_const_ll;
    boolean_T SRFlipFlop_initial_condition;
    boolean_T SRFlipFlop_initial_condition_c;
    boolean_T ConfirmNode_isRisingEdge;
    boolean_T ConfirmNode2_isRisingEdge;
    boolean_T ConfirmNode1_isRisingEdge;
    boolean_T ConfirmNode_isRisingEdge_k;
    boolean_T ConfirmNode2_isRisingEdge_j;
    boolean_T ConfirmNode_isRisingEdge_o;
    boolean_T PulseNode_isRisingEdge;
    boolean_T PulseNode1_isRisingEdge;
    boolean_T ConfirmNode1_isRisingEdge_k;
    boolean_T ConfirmNode_isRisingEdge_j;
    boolean_T ConfirmNode_isRisingEdge_c;
    boolean_T PulseNode3_isRisingEdge;
    boolean_T PulseNode2_isRisingEdge;
    boolean_T PulseNode1_isRisingEdge_c;
    boolean_T PulseNode_isRisingEdge_n;
    prim_outputs out_Y0;
    base_prim_out_bus Constant4_Value;
    real_T Constant5_Value;
    real_T Constant6_Value;
    real_T Constant9_Value;
    real_T uDLookupTable_tableData[5];
    real_T uDLookupTable_bp01Data[5];
    real_T Constant2_Value;
    real_T Constant1_Value;
    real_T Constant4_Value_a;
    real_T Constant3_Value;
    real_T Constant_Value;
    real_T Constant2_Value_l;
    real_T Constant3_Value_h;
    real_T Constant10_Value;
    real_T Constant11_Value;
    real_T Constant1_Value_n;
    real_T Constant2_Value_k;
    real_T Constant3_Value_g;
    real_T Constant4_Value_i;
    real_T Constant5_Value_n;
    real_T Constant6_Value_f;
    real_T Constant7_Value;
    real_T Constant8_Value;
    real_T Constant9_Value_n;
    real_T Constant_Value_b;
    real_T Constant_Value_p;
    real_T Saturation_UpperSat;
    real_T Saturation_LowerSat;
    real_T Constant1_Value_p;
    real_T Saturation1_UpperSat;
    real_T Saturation1_LowerSat;
    real_T alphamax_tableData[24];
    real_T alphamax_bp01Data[4];
    real_T alphamax_bp02Data[6];
    real_T alphaprotection_tableData[24];
    real_T alphaprotection_bp01Data[4];
    real_T alphaprotection_bp02Data[6];
    real_T Constant5_Value_k;
    real_T Constant6_Value_b;
    real_T Constant7_Value_g;
    real_T Constant8_Value_h;
    real_T Gain1_Gain;
    real_T uDLookupTable1_tableData[4];
    real_T uDLookupTable1_bp01Data[4];
    real_T uDLookupTable2_tableData[4];
    real_T uDLookupTable2_bp01Data[4];
    real_T uDLookupTable_tableData_n[4];
    real_T uDLookupTable_bp01Data_m[4];
    real_T Constant_Value_a;
    real_T Constant_Value_c;
    real_T Gain_Gain;
    real_T Saturation2_UpperSat;
    real_T Saturation2_LowerSat;
    real_T Saturation1_UpperSat_a;
    real_T Saturation1_LowerSat_p;
    real_T Gain3_Gain;
    real_T Saturation3_UpperSat;
    real_T Saturation3_LowerSat;
    real_T Saturation4_UpperSat;
    real_T Saturation4_LowerSat;
    real_T Gain4_Gain;
    real_T Saturation5_UpperSat;
    real_T Saturation5_LowerSat;
    real_T Saturation6_UpperSat;
    real_T Saturation6_LowerSat;
    real_T Gain5_Gain;
    real_T Constant8_Value_d;
    real_T Constant_Value_g;
    real_T Constant_Value_af;
    real32_T Constant10_Value_l;
    real32_T Constant9_Value_m;
    real32_T Constant8_Value_hh;
    real32_T Constant7_Value_j;
    real32_T Constant6_Value_k;
    real32_T Constant5_Value_g;
    real32_T Constant4_Value_a5;
    real32_T Constant3_Value_ge;
    real32_T Constant2_Value_c;
    real32_T Constant1_Value_nj;
    real32_T Constant14_Value;
    real32_T Constant15_Value;
    real32_T Constant24_Value;
    real32_T Constant23_Value;
    real32_T Constant26_Value;
    real32_T Constant25_Value;
    real32_T Constant28_Value;
    real32_T Constant27_Value;
    real32_T Constant30_Value;
    real32_T Constant29_Value;
    real32_T Constant32_Value;
    real32_T Constant13_Value;
    real32_T Constant31_Value;
    real32_T Constant33_Value;
    real32_T Constant34_Value;
    real32_T Constant35_Value;
    real32_T Constant12_Value;
    real32_T Constant11_Value_l;
    real32_T Constant20_Value;
    real32_T Gain_Gain_b;
    real32_T Gain1_Gain_f;
    real32_T Gain2_Gain;
    real32_T Gain3_Gain_g;
    real32_T Gain4_Gain_l;
    uint32_T alphamax_maxIndex[2];
    uint32_T alphaprotection_maxIndex[2];
    boolean_T Constant1_Value_b;
    boolean_T Constant_Value_ad;
    boolean_T Constant_Value_o;
    boolean_T Delay_InitialCondition;
    boolean_T Delay1_InitialCondition;
    boolean_T reset_Value;
    boolean_T reset_Value_j;
    boolean_T Constant_Value_h;
    boolean_T Logic_table[16];
    boolean_T Logic_table_h[16];
    boolean_T Delay1_InitialCondition_n;
    boolean_T Delay_InitialCondition_d;
    boolean_T reset_Value_f;
    boolean_T Constant7_Value_n;
    boolean_T reset_Value_l;
    boolean_T Constant16_Value;
    boolean_T Constant17_Value;
    boolean_T Constant18_Value;
    boolean_T Constant19_Value;
    boolean_T Constant21_Value;
    boolean_T Constant22_Value;
    boolean_T Constant1_Value_f;
    boolean_T Constant_Value_ba;
  };

  A380PrimComputer(A380PrimComputer const&) = delete;
  A380PrimComputer& operator= (A380PrimComputer const&) & = delete;
  A380PrimComputer(A380PrimComputer &&) = delete;
  A380PrimComputer& operator= (A380PrimComputer &&) = delete;
  void setExternalInputs(const ExternalInputs_A380PrimComputer_T *pExternalInputs_A380PrimComputer_T)
  {
    A380PrimComputer_U = *pExternalInputs_A380PrimComputer_T;
  }

  const ExternalOutputs_A380PrimComputer_T &getExternalOutputs() const
  {
    return A380PrimComputer_Y;
  }

  void initialize();
  void step();
  static void terminate();
  A380PrimComputer();
  ~A380PrimComputer();
 private:
  ExternalInputs_A380PrimComputer_T A380PrimComputer_U;
  ExternalOutputs_A380PrimComputer_T A380PrimComputer_Y;
  BlockIO_A380PrimComputer_T A380PrimComputer_B;
  D_Work_A380PrimComputer_T A380PrimComputer_DWork;
  static Parameters_A380PrimComputer_T A380PrimComputer_P;
  static void A380PrimComputer_RateLimiter_Reset(rtDW_RateLimiter_A380PrimComputer_T *localDW);
  static void A380PrimComputer_RateLimiter(real_T rtu_u, real_T rtu_up, real_T rtu_lo, real_T rtu_Ts, real_T rtu_init,
    real_T *rty_Y, rtDW_RateLimiter_A380PrimComputer_T *localDW);
  static void A380PrimComputer_RateLimiter_b_Reset(rtDW_RateLimiter_A380PrimComputer_g_T *localDW);
  static void A380PrimComputer_RateLimiter_a(real_T rtu_u, real_T rtu_up, real_T rtu_lo, real_T rtu_Ts, real_T rtu_init,
    boolean_T rtu_reset, real_T *rty_Y, rtDW_RateLimiter_A380PrimComputer_g_T *localDW);
  static void A380PrimComputer_RateLimiter_bb_Reset(rtDW_RateLimiter_A380PrimComputer_d_T *localDW);
  static void A380PrimComputer_RateLimiter_m(real_T rtu_u, real_T rtu_up, real_T rtu_lo, real_T rtu_Ts, real_T rtu_init,
    boolean_T rtu_reset, real_T *rty_Y, rtDW_RateLimiter_A380PrimComputer_d_T *localDW);
  static void A380PrimComputer_Spoiler12SpeedbrakeGain(real_T rtu_spdBrkDeflection, real_T *rty_spdBrkDeflectionOut);
  static void A380PrimComputer_Spoiler345Computation(real_T rtu_xiSplr, real_T rtu_speedBrakeDeflection, real_T
    *rty_leftCommand, real_T *rty_rightCommand);
  static void A380PrimComputer_MATLABFunction(const base_arinc_429 *rtu_u, boolean_T *rty_y);
  static void A380PrimComputer_MATLABFunction_e(const base_arinc_429 *rtu_u, real_T rtu_bit, uint32_T *rty_y);
  static void A380PrimComputer_MATLABFunction_o(boolean_T rtu_bit1, boolean_T rtu_bit2, boolean_T rtu_bit3, boolean_T
    rtu_bit4, boolean_T rtu_bit5, boolean_T rtu_bit6, real_T *rty_handleIndex);
  static void A380PrimComputer_LagFilter_Reset(rtDW_LagFilter_A380PrimComputer_T *localDW);
  static void A380PrimComputer_LagFilter(real_T rtu_U, real_T rtu_C1, real_T rtu_dt, real_T *rty_Y,
    rtDW_LagFilter_A380PrimComputer_T *localDW);
  static void A380PrimComputer_MATLABFunction_m_Reset(rtDW_MATLABFunction_A380PrimComputer_j_T *localDW);
  static void A380PrimComputer_MATLABFunction_m(boolean_T rtu_u, boolean_T rtu_isRisingEdge, boolean_T *rty_y,
    rtDW_MATLABFunction_A380PrimComputer_j_T *localDW);
  static void A380PrimComputer_MATLABFunction_p_Reset(rtDW_MATLABFunction_A380PrimComputer_k_T *localDW);
  static void A380PrimComputer_MATLABFunction_c(boolean_T rtu_u, real_T rtu_Ts, boolean_T rtu_isRisingEdge, real_T
    rtu_timeDelay, boolean_T *rty_y, rtDW_MATLABFunction_A380PrimComputer_k_T *localDW);
  static void A380PrimComputer_MATLABFunction_j_Reset(rtDW_MATLABFunction_A380PrimComputer_km_T *localDW);
  static void A380PrimComputer_MATLABFunction_j(real_T rtu_u, real_T rtu_highTrigger, real_T rtu_lowTrigger, boolean_T
    *rty_y, rtDW_MATLABFunction_A380PrimComputer_km_T *localDW);
  static void A380PrimComputer_MATLABFunction_ek(boolean_T rtu_bit1, boolean_T rtu_bit2, boolean_T rtu_bit3, boolean_T
    rtu_valid, a380_pitch_efcs_law *rty_law);
  static void A380PrimComputer_GetIASforMach4(real_T rtu_m, real_T rtu_m_t, real_T rtu_v, real_T *rty_v_t);
  static void A380PrimComputer_RateLimiter_e_Reset(rtDW_RateLimiter_A380PrimComputer_b_T *localDW);
  static void A380PrimComputer_RateLimiter_n(real_T rtu_u, real_T rtu_up, real_T rtu_lo, real_T rtu_Ts, boolean_T
    rtu_reset, real_T *rty_Y, rtDW_RateLimiter_A380PrimComputer_b_T *localDW);
  static void A380PrimComputer_MATLABFunction_cw(const boolean_T rtu_u[19], real32_T *rty_y);
  static void A380PrimComputer_MATLABFunction_ei(a380_pitch_efcs_law rtu_law, boolean_T *rty_bit1, boolean_T *rty_bit2,
    boolean_T *rty_bit3);
  static void A380PrimComputer_MATLABFunction2(a380_lateral_efcs_law rtu_law, boolean_T *rty_bit1, boolean_T *rty_bit2);
  A380LateralDirectLaw LawMDLOBJ1;
  A380LateralNormalLaw LawMDLOBJ2;
  A380PitchAlternateLaw LawMDLOBJ3;
  A380PitchDirectLaw LawMDLOBJ4;
  A380PitchNormalLaw LawMDLOBJ5;
};

#endif

