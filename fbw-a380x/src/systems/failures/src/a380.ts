// Copyright (c) 2023-2024 FlyByWire Simulations
// SPDX-License-Identifier: GPL-3.0
// One can rightfully argue that this constant shouldn't be located in @flybywiresim/failures.
// Once we create an A320 specific package, such as @flybywiresim/a320, we can move it there.
import { FailureDefinition } from '@flybywiresim/fbw-sdk';

export const A380Failure = Object.freeze({
  FmcA: 22000,
  FmcB: 22001,
  FmcC: 22002,

  AudioManagementUnit1: 23000,
  AudioManagementUnit2: 23001,
  RadioManagementPanel1: 23002,
  RadioManagementPanel2: 23003,
  RadioManagementPanel3: 23004,
  Vhf1: 23005,
  Vhf2: 23006,
  Vhf3: 23007,

  TransformerRectifier1: 24000,
  TransformerRectifier2: 24001,
  TransformerRectifierEssential: 24002,

  GreenReservoirLeak: 29000,
  YellowReservoirLeak: 29001,
  GreenReservoirAirLeak: 29002,
  YellowReservoirAirLeak: 29003,
  GreenReservoirReturnLeak: 29004,
  YellowReservoirReturnLeak: 29005,
  GreenElecPumpAOHeat: 29006,
  GreenElecPumpBOHeat: 29007,
  YellowElecPumpAOHeat: 29008,
  YellowElecPumpBOHeat: 29009,
  EnginePump1AOHeat: 29010,
  EnginePump1BOHeat: 29011,
  EnginePump2AOHeat: 29012,
  EnginePump2BOHeat: 29013,
  EnginePump3AOHeat: 29014,
  EnginePump3BOHeat: 29015,
  EnginePump4AOHeat: 29016,
  EnginePump4BOHeat: 29017,

  LeftPfdDisplay: 31000,
  RightPfdDisplay: 31001,

  LgciuPowerSupply1: 32000,
  LgciuPowerSupply2: 32001,
  LgciuInternalError1: 32002,
  LgciuInternalError2: 32003,

  GearProxSensorDamageGearUplockLeft1: 32004,
  GearProxSensorDamageDoorDownlockRight2: 32005,
  GearProxSensorDamageGearUplockNose1: 32006,
  GearProxSensorDamageDoorUplockLeft2: 32007,

  RadioAltimeter1: 34000,
  RadioAltimeter2: 34001,
  Transponder1: 34002,
  Transponder2: 34003,
});

export const A380FailureDefinitions: FailureDefinition[] = [
  [22, A380Failure.FmcA, 'FMC-A'],
  [22, A380Failure.FmcB, 'FMC-B'],
  [22, A380Failure.FmcC, 'FMC-C'],

  [23, A380Failure.AudioManagementUnit1, 'AMU 1'],
  [23, A380Failure.AudioManagementUnit2, 'AMU 2'],
  [23, A380Failure.RadioManagementPanel1, 'RMP 1'],
  [23, A380Failure.RadioManagementPanel2, 'RMP 2'],
  [23, A380Failure.RadioManagementPanel3, 'RMP 3'],
  [23, A380Failure.Vhf1, 'VHF 1'],
  [23, A380Failure.Vhf2, 'VHF 2'],
  [23, A380Failure.Vhf3, 'VHF 3'],

  [24, A380Failure.TransformerRectifier1, 'TR 1'],
  [24, A380Failure.TransformerRectifier2, 'TR 2'],
  [24, A380Failure.TransformerRectifierEssential, 'ESS TR'],

  [29, A380Failure.GreenReservoirLeak, 'Green reservoir leak'],
  [29, A380Failure.YellowReservoirLeak, 'Yellow reservoir leak'],
  [29, A380Failure.GreenReservoirAirLeak, 'Green reservoir air leak'],
  [29, A380Failure.YellowReservoirAirLeak, 'Yellow reservoir air leak'],
  [29, A380Failure.GreenReservoirReturnLeak, 'Green reservoir return leak'],
  [29, A380Failure.YellowReservoirReturnLeak, 'Yellow reservoir return leak'],
  [29, A380Failure.GreenElecPumpAOHeat, 'Green A elec pump overheat'],
  [29, A380Failure.GreenElecPumpBOHeat, 'Green B elec pump overheat'],
  [29, A380Failure.YellowElecPumpAOHeat, 'Yellow A elec pump overheat'],
  [29, A380Failure.YellowElecPumpBOHeat, 'Yellow B elec pump overheat'],
  [29, A380Failure.EnginePump1AOHeat, 'Engine 1 pump A overheat'],
  [29, A380Failure.EnginePump1BOHeat, 'Engine 1 pump B overheat'],
  [29, A380Failure.EnginePump2AOHeat, 'Engine 2 pump A overheat'],
  [29, A380Failure.EnginePump2BOHeat, 'Engine 2 pump B overheat'],
  [29, A380Failure.EnginePump3AOHeat, 'Engine 3 pump A overheat'],
  [29, A380Failure.EnginePump3BOHeat, 'Engine 3 pump B overheat'],
  [29, A380Failure.EnginePump4AOHeat, 'Engine 4 pump A overheat'],
  [29, A380Failure.EnginePump4BOHeat, 'Engine 4 pump B overheat'],

  [31, A380Failure.LeftPfdDisplay, 'Captain PFD display'],
  [31, A380Failure.RightPfdDisplay, 'F/O PFD display'],

  [32, A380Failure.LgciuPowerSupply1, 'LGCIU 1 Power supply'],
  [32, A380Failure.LgciuPowerSupply2, 'LGCIU 2 Power supply'],
  [32, A380Failure.LgciuInternalError1, 'LGCIU 1 Internal error'],
  [32, A380Failure.LgciuInternalError2, 'LGCIU 2 Internal error'],

  [32, A380Failure.GearProxSensorDamageGearUplockNose1, 'Proximity sensor damage uplock nose gear #1'],

  [34, A380Failure.RadioAltimeter1, 'RA 1'],
  [34, A380Failure.RadioAltimeter2, 'RA 2'],
  [34, A380Failure.Transponder1, 'XPDR 1'],
  [34, A380Failure.Transponder2, 'XPDR 2'],
];
