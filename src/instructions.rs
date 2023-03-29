use std::fmt::Display;

use phf::{phf_map, phf_set};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DataType {
    Number,
    Register,
    Device,
    LogicType,
    SlotLogicType,
    Name,
    BatchMode,
}

#[derive(Debug)]
pub(crate) struct Union<'a>(pub(crate) &'a [DataType]);

#[derive(Debug)]
pub(crate) struct InstructionSignature(pub(crate) &'static [Union<'static>]);

const REGISTER: Union = Union(&[DataType::Register]);
const DEVICE: Union = Union(&[DataType::Device]);
const VALUE: Union = Union(&[DataType::Register, DataType::Number]);
const LOGIC_TYPE: Union = Union(&[DataType::LogicType]);
const SLOT_LOGIC_TYPE: Union = Union(&[DataType::SlotLogicType]);
const BATCH_MODE: Union = Union(&[DataType::BatchMode, DataType::Number, DataType::Register]);

pub(crate) const INSTRUCTIONS: phf::Map<&'static str, InstructionSignature> = phf_map! {
    "alias" => InstructionSignature(&[Union(&[DataType::Name]), Union(&[DataType::Register, DataType::Device])]),
    "label" => InstructionSignature(&[Union(&[DataType::Name]), Union(&[DataType::Register, DataType::Device])]),
    "define" => InstructionSignature(&[Union(&[DataType::Name]), Union(&[DataType::Number])]),
    "bdns" => InstructionSignature(&[DEVICE,VALUE]),
    "bdnsal" => InstructionSignature(&[DEVICE,VALUE]),
    "bdse" => InstructionSignature(&[DEVICE,VALUE]),
    "bdseal" => InstructionSignature(&[DEVICE,VALUE]),
    "brdns" => InstructionSignature(&[DEVICE,VALUE]),
    "brdse" => InstructionSignature(&[DEVICE,VALUE]),
    "l" => InstructionSignature(&[REGISTER,DEVICE,LOGIC_TYPE]),
    "lb" => InstructionSignature(&[REGISTER,VALUE,LOGIC_TYPE,BATCH_MODE]),
    // "lr" => InstructionSignature(&[REGISTER,DEVICE,reagentMode,reagent]),
    "ls" => InstructionSignature(&[REGISTER,VALUE,VALUE,SLOT_LOGIC_TYPE]),
    "s" => InstructionSignature(&[DEVICE,LOGIC_TYPE,VALUE]),
    "sb" => InstructionSignature(&[VALUE,LOGIC_TYPE,REGISTER]),
    "bap" => InstructionSignature(&[VALUE,VALUE,VALUE,VALUE]),
    "bapal" => InstructionSignature(&[VALUE,VALUE,VALUE,VALUE]),
    "bapz" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bapzal" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "beq" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "beqal" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "beqz" => InstructionSignature(&[VALUE,VALUE]),
    "beqzal" => InstructionSignature(&[VALUE,VALUE]),
    "bge" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bgeal" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bgez" => InstructionSignature(&[VALUE,VALUE]),
    "bgezal" => InstructionSignature(&[VALUE,VALUE]),
    "bgt" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bgtal" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bgtz" => InstructionSignature(&[VALUE,VALUE]),
    "bgtzal" => InstructionSignature(&[VALUE,VALUE]),
    "ble" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bleal" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "blez" => InstructionSignature(&[VALUE,VALUE]),
    "blezal" => InstructionSignature(&[VALUE,VALUE]),
    "blt" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bltal" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bltz" => InstructionSignature(&[VALUE,VALUE]),
    "bltzal" => InstructionSignature(&[VALUE,VALUE]),
    "bna" => InstructionSignature(&[VALUE,VALUE,VALUE,VALUE]),
    "bnaal" => InstructionSignature(&[VALUE,VALUE,VALUE,VALUE]),
    "bnaz" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bnazal" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bne" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bneal" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "bnez" => InstructionSignature(&[VALUE,VALUE]),
    "bnezal" => InstructionSignature(&[VALUE,VALUE]),
    "brap" => InstructionSignature(&[VALUE,VALUE,VALUE,VALUE]),
    "brapz" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "brnaz" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "breq" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "breqz" => InstructionSignature(&[VALUE,VALUE]),
    "brge" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "brgez" => InstructionSignature(&[VALUE,VALUE]),
    "brgt" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "brgtz" => InstructionSignature(&[VALUE,VALUE]),
    "brle" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "brlez" => InstructionSignature(&[VALUE,VALUE]),
    "brlt" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "brltz" => InstructionSignature(&[VALUE,VALUE]),
    "brna" => InstructionSignature(&[VALUE,VALUE,VALUE,VALUE]),
    "brne" => InstructionSignature(&[VALUE,VALUE,VALUE]),
    "brnez" => InstructionSignature(&[VALUE,VALUE]),
    "j" => InstructionSignature(&[VALUE]),
    "jal" => InstructionSignature(&[VALUE]),
    "jr" => InstructionSignature(&[VALUE]),
    "sap" => InstructionSignature(&[REGISTER,VALUE,VALUE,VALUE]),
    "sapz" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "sdns" => InstructionSignature(&[REGISTER,DEVICE]),
    "sdse" => InstructionSignature(&[REGISTER,DEVICE]),
    "select" => InstructionSignature(&[REGISTER,VALUE,VALUE,VALUE]),
    "seq" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "seqz" => InstructionSignature(&[REGISTER,VALUE]),
    "sge" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "sgez" => InstructionSignature(&[REGISTER,VALUE]),
    "sgt" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "sgtz" => InstructionSignature(&[REGISTER,VALUE]),
    "sle" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "slez" => InstructionSignature(&[REGISTER,VALUE]),
    "slt" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "sltz" => InstructionSignature(&[REGISTER,VALUE]),
    "sna" => InstructionSignature(&[REGISTER,VALUE,VALUE,VALUE]),
    "snaz" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "sne" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "snez" => InstructionSignature(&[REGISTER,VALUE]),
    "abs" => InstructionSignature(&[REGISTER,VALUE]),
    "acos" => InstructionSignature(&[REGISTER,VALUE]),
    "add" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "asin" => InstructionSignature(&[REGISTER,VALUE]),
    "atan" => InstructionSignature(&[REGISTER,VALUE]),
    "atan2" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "ceil" => InstructionSignature(&[REGISTER,VALUE]),
    "cos" => InstructionSignature(&[REGISTER,VALUE]),
    "div" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "exp" => InstructionSignature(&[REGISTER,VALUE]),
    "floor" => InstructionSignature(&[REGISTER,VALUE]),
    "log" => InstructionSignature(&[REGISTER,VALUE]),
    "max" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "min" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "mod" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "mul" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "rand" => InstructionSignature(&[REGISTER]),
    "round" => InstructionSignature(&[REGISTER,VALUE]),
    "sin" => InstructionSignature(&[REGISTER,VALUE]),
    "sqrt" => InstructionSignature(&[REGISTER,VALUE]),
    "sub" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "tan" => InstructionSignature(&[REGISTER,VALUE]),
    "trunc" => InstructionSignature(&[REGISTER,VALUE]),
    "and" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "nor" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "or" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "xor" => InstructionSignature(&[REGISTER,VALUE,VALUE]),
    "peek" => InstructionSignature(&[REGISTER]),
    "pop" => InstructionSignature(&[REGISTER]),
    "push" => InstructionSignature(&[VALUE]),
    "hcf" => InstructionSignature(&[]),
    "move" => InstructionSignature(&[REGISTER,VALUE]),
    "sleep" => InstructionSignature(&[VALUE]),
    "yield" => InstructionSignature(&[]),
};

pub(crate) const LOGIC_TYPES: phf::Set<&'static str> = phf_set! {
    "Power",
    "Open",
    "Mode",
    "Error",
    "Lock",
    "Pressure",
    "Temperature",
    "PressureExternal",
    "PressureInternal",
    "Activate",
    "Charge",
    "Setting",
    "Reagents",
    "RatioOxygen",
    "RatioCarbonDioxide",
    "RatioNitrogen",
    "RatioPollutant",
    "RatioVolatiles",
    "RatioWater",
    "Horizontal",
    "Vertical",
    "SolarAngle",
    "Maximum",
    "Ratio",
    "PowerPotential",
    "PowerActual",
    "Quantity",
    "On",
    "ImportQuantity",
    "ImportSlotOccupant",
    "ExportQuantity",
    "ExportSlotOccupant",
    "RequiredPower",
    "HorizontalRatio",
    "VerticalRatio",
    "PowerRequired",
    "Idle",
    "Color",
    "ElevatorSpeed",
    "ElevatorLevel",
    "RecipeHash",
    "ExportSlotHash",
    "ImportSlotHash",
    "PlantHealth1",
    "PlantHealth2",
    "PlantHealth3",
    "PlantHealth4",
    "PlantGrowth1",
    "PlantGrowth2",
    "PlantGrowth3",
    "PlantGrowth4",
    "PlantEfficiency1",
    "PlantEfficiency2",
    "PlantEfficiency3",
    "PlantEfficiency4",
    "PlantHash1",
    "PlantHash2",
    "PlantHash3",
    "PlantHash4",
    "RequestHash",
    "CompletionRatio",
    "ClearMemory",
    "ExportCount",
    "ImportCount",
    "PowerGeneration",
    "TotalMoles",
    "Volume",
    "Plant",
    "Harvest",
    "Output",
    "PressureSetting",
    "TemperatureSetting",
    "TemperatureExternal",
    "Filtration",
    "AirRelease",
    "PositionX",
    "PositionY",
    "PositionZ",
    "VelocityMagnitude",
    "VelocityRelativeX",
    "VelocityRelativeY",
    "VelocityRelativeZ",
    "RatioNitrousOxide",
    "PrefabHash",
    "ForceWrite",
    "SignalStrength",
    "SignalID",
    "TargetX",
    "TargetY",
    "TargetZ",
    "SettingInput",
    "SettingOutput",
    "CurrentResearchPodType",
    "ManualResearchRequiredPod",
    "MineablesInVicinity",
    "MineablesInQueue",
    "NextWeatherEventTime",
    "Combustion",
    "Fuel",
    "ReturnFuelCost",
    "CollectableGoods",
    "Time",
    "Bpm",
    "EnvironmentEfficiency",
    "WorkingGasEfficiency",
    "PressureInput",
    "TemperatureInput",
    "RatioOxygenInput",
    "RatioCarbonDioxideInput",
    "RatioNitrogenInput",
    "RatioPollutantInput",
    "RatioVolatilesInput",
    "RatioWaterInput",
    "RatioNitrousOxideInput",
    "TotalMolesInput",
    "PressureInput2",
    "TemperatureInput2",
    "RatioOxygenInput2",
    "RatioCarbonDioxideInput2",
    "RatioNitrogenInput2",
    "RatioPollutantInput2",
    "RatioVolatilesInput2",
    "RatioWaterInput2",
    "RatioNitrousOxideInput2",
    "TotalMolesInput2",
    "PressureOutput",
    "TemperatureOutput",
    "RatioOxygenOutput",
    "RatioCarbonDioxideOutput",
    "RatioNitrogenOutput",
    "RatioPollutantOutput",
    "RatioVolatilesOutput",
    "RatioWaterOutput",
    "RatioNitrousOxideOutput",
    "TotalMolesOutput",
    "PressureOutput2",
    "TemperatureOutput2",
    "RatioOxygenOutput2",
    "RatioCarbonDioxideOutput2",
    "RatioNitrogenOutput2",
    "RatioPollutantOutput2",
    "RatioVolatilesOutput2",
    "RatioWaterOutput2",
    "RatioNitrousOxideOutput2",
    "TotalMolesOutput2",
    "CombustionInput",
    "CombustionInput2",
    "CombustionOutput",
    "CombustionOutput2",
    "OperationalTemperatureEfficiency",
    "TemperatureDifferentialEfficiency",
    "PressureEfficiency",
    "CombustionLimiter",
    "Throttle",
    "Rpm",
    "Stress",
    "InterrogationProgress",
    "TargetPadIndex",
    "SizeX",
    "SizeY",
    "SizeZ",
    "MinimumWattsToContact",
    "WattsReachingContact",
};

pub(crate) const SLOT_LOGIC_TYPES: phf::Set<&'static str> = phf_set! {
    "Occupied",
    "OccupantHash",
    "Quantity",
    "Damage",
    "Efficiency",
    "Health",
    "Growth",
    "Pressure",
    "Temperature",
    "Charge",
    "ChargeRatio",
    "Class",
    "PressureWaste",
    "PressureAir",
    "MaxQuantity",
    "Mature",
    "PrefabHash",
    "Seeding",
};

pub(crate) const BATCH_MODES: phf::Set<&'static str> = phf_set! {
    "Average",
    "Sum",
    "Minimum",
    "Maximum",
};

pub(crate) const BATCH_MODE_LOOKUP: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Average",
    1u8 => "Sum",
    2u8 => "Minimum",
    3u8 => "Maximum",
};

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match *self {
            DataType::Number => "num",
            DataType::Register => "r?",
            DataType::Device => "d?",
            DataType::LogicType => "type",
            DataType::SlotLogicType => "slotType",
            DataType::Name => "name",
            DataType::BatchMode => "batchMode",
        };
        write!(f, "{}", val)
    }
}

impl Display for InstructionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for parameter in self.0 {
            write!(f, " {parameter}")?;
        }
        Ok(())
    }
}

impl<'a> Display for Union<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let has_parens = self.0.len() != 1;
        if has_parens {
            write!(f, "(")?;
        }
        let mut first = true;
        for item in self.0.iter() {
            if !first {
                write!(f, "|")?;
            }
            first = false;
            item.fmt(f)?;
        }
        if has_parens {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl<'a> From<&'a [DataType]> for Union<'a> {
    fn from(value: &'a [DataType]) -> Self {
        Union(value)
    }
}

impl<'a> Union<'a> {
    pub(crate) fn match_type(&self, typ: DataType) -> bool {
        for x in self.0 {
            if *x == typ {
                return true;
            }
        }
        false
    }

    pub(crate) fn match_union(&self, types: &Union) -> bool {
        for typ in self.0 {
            for typ2 in types.0 {
                if typ == typ2 {
                    return true;
                }
            }
        }
        false
    }
}

// Taken directly from the game's rocketstation_Data/StreamingAssets/Language/english.xml
// with slight changes
pub(crate) const INSTRUCTION_DOCS: phf::Map<&'static str, &'static str> = phf_map! {
    "l" => "Loads device var to register.",
    "lb" => "Loads var from all output network devices with provided type hash using the provide batch mode. Average (0), Sum (1), Minimum (2), Maximum (3). Can use either the word, or the number.",
    "s" => "Stores register value to var on device.",
    "sb" => "Stores register value to var on all output network devices with provided type hash.",
    "ls" => "Loads slot var on device to register.",
    // "lr" => "Loads reagent of device's reagentMode to register. Contents (0), Required (1), Recipe (2). Can use either the word, or the number.",
    "alias" => "Labels register or device reference with name, device references also affect what shows on the screws on the IC base.",
    "define" => "Creates a label that will be replaced throughout the program with the provided value.",
    "move" => "Register = provided num or register value.",
    "add" => "Register = a + b.",
    "sub" => "Register = a - b.",
    "sdse" => "Register = 1 if device is set, otherwise 0.",
    "sdns" => "Register = 1 if device is not set, otherwise 0",
    "slt" => "Register = 1 if a &lt; b, otherwise 0",
    "sgt" => "Register = 1 if a &gt; b, otherwise 0",
    "sle" => "Register = 1 if a &lt;= b, otherwise 0",
    "sge" => "Register = 1 if a &gt;= b, otherwise 0",
    "seq" => "Register = 1 if a == b, otherwise 0",
    "sne" => "Register = 1 if a != b, otherwise 0",
    "sap" => "Register = 1 if abs(a - b) &lt;= max(c * max(abs(a), abs(b)), float.epsilon * 8), otherwise 0",
    "sna" => "Register = 1 if abs(a - b) &gt; max(c * max(abs(a), abs(b)), float.epsilon * 8), otherwise 0",
    "sltz" => "Register = 1 if a &lt; 0, otherwise 0",
    "sgtz" => "Register = 1 if a &gt; 0, otherwise 0",
    "slez" => "Register = 1 if a &lt;= 0, otherwise 0",
    "sgez" => "Register = 1 if a &gt;= 0, otherwise 0",
    "seqz" => "Register = 1 if a == 0, otherwise 0",
    "snez" => "Register = 1 if a != 0, otherwise 0",
    "sapz" => "Register = 1 if abs(a) <= max(b * abs(a), float.epsilon * 8), otherwise 0",
    "snaz" => "Register = 1 if abs(a) > max(b * abs(a), float.epsilon * 8), otherwise 0",
    "and" => "Register = 1 if a and b not zero, otherwise 0",
    "or" => "Register = 1 if a and/or b not 0, otherwise 0",
    "xor" => "Register = 1 if either a or b not 0, otherwise 0",
    "nor" => "Register = 1 if a and b are 0, otherwise 0",
    "mul" => "Register = a * b",
    "div" => "Register = a / b",
    "mod" => "Register = a mod b (note: NOT a % b)",
    "j" => "Jump execution to line a",
    "bdse" => "Branch to line a if device d is set",
    "bdns" => "Branch to line a if device d isn't set",
    "blt" => "Branch to line c if a &lt; b",
    "bgt" => "Branch to line c if a &gt; b",
    "ble" => "Branch to line c if a &lt;= b",
    "bge" => "Branch to line c if a &gt;= b",
    "beq" => "Branch to line c if a == b",
    "bne" => "Branch to line c if a != b",
    "bap" => "Branch to line d if abs(a - b) &lt;= max(c * max(abs(a), abs(b)), float.epsilon * 8)",
    "bna" => "Branch to line d if abs(a - b) &gt; max(c * max(abs(a), abs(b)), float.epsilon * 8)",
    "bltz" => "Branch to line b if a &lt; 0",
    "bgez" => "Branch to line b if a &gt;= 0",
    "blez" => "Branch to line b if a &lt;= 0",
    "bgtz" => "Branch to line b if a &gt; 0",
    "beqz" => "Branch to line b if a == 0",
    "bnez" => "Branch to line b if a != 0",
    "bapz" => "Branch to line c if abs(a) &lt;= float.epsilon * 8",
    "bnaz" => "Branch to line c if abs(a) &gt; float.epsilon * 8",
    "jr" => "Relative jump to line a",
    "brdse" => "Relative jump to line a if device is set",
    "brdns" => "Relative jump to line a if device is not set",
    "brlt" => "Relative jump to line c if a &lt; b",
    "brgt" => "Relative jump to line c if a &gt; b",
    "brle" => "Relative jump to line c if a &lt;= b",
    "brge" => "Relative jump to line c if a &gt;= b",
    "breq" => "Relative branch to line c if a == b",
    "brne" => "Relative branch to line c if a != b",
    "brap" => "Relative branch to line d if abs(a - b) &lt;= max(c * max(abs(a), abs(b)), float.epsilon * 8)",
    "brna" => "Relative branch to line d if abs(a - b) &gt; max(c * max(abs(a), abs(b)), float.epsilon * 8)",
    "brltz" => "Relative branch to line b if a &lt; 0",
    "brgez" => "Relative branch to line b if a &gt;= 0",
    "brlez" => "Relative branch to line b if a &lt;= 0",
    "brgtz" => "Relative branch to line b if a &gt; 0",
    "breqz" => "Relative branch to line b if a == 0",
    "brnez" => "Relative branch to line b if a != 0",
    "brapz" => "Relative branch to line c if abs(a) &lt;= float.epsilon * 8",
    "brnaz" => "Relative branch to line c if abs(a) &gt; float.epsilon * 8",
    "jal" => "Jump execution to line a and store next line number in ra",
    "bdseal" => "Jump execution to line a and store next line number if device is set",
    "bdnsal" => "Jump execution to line a and store next line number if device is not set",
    "bltal" => "Branch to line c if a &lt; b and store next line number in ra",
    "bgtal" => "Branch to line c if a &gt; b and store next line number in ra",
    "bleal" => "Branch to line c if a &lt;= b and store next line number in ra",
    "bgeal" => "Branch to line c if a &gt;= b and store next line number in ra",
    "beqal" => "Branch to line c if a == b and store next line number in ra",
    "bneal" => "Branch to line c if a != b and store next line number in ra",
    "bapal" => "Branch to line d if abs(a - b) &lt;= max(c * max(abs(a), abs(b)), float.epsilon * 8) and store next line number in ra",
    "bnaal" => "Branch to line d if abs(a - b) &lt;= max(c * max(abs(a), abs(b)), float.epsilon * 8) and store next line number in ra",
    "bltzal" => "Branch to line b if a &lt; 0 and store next line number in ra",
    "bgezal" => "Branch to line b if a &gt;= 0 and store next line number in ra",
    "blezal" => "Branch to line b if a &lt;= 0 and store next line number in ra",
    "bgtzal" => "Branch to line b if a &gt; 0 and store next line number in ra",
    "beqzal" => "Branch to line b if a == 0 and store next line number in ra",
    "bnezal" => "Branch to line b if a != 0 and store next line number in ra",
    "bapzal" => "Branch to line c if abs(a) &lt;= float.epsilon * 8",
    "bnazal" => "Branch to line c if abs(a) &gt; float.epsilon * 8",
    "sqrt" => "Register = square root of a",
    "round" => "Register = a rounded to nearest integer",
    "trunc" => "Register = a with fractional part removed",
    "ceil" => "Register = smallest integer greater than a",
    "floor" => "Register = largest integer less than a",
    "max" => "Register = max of a or b",
    "min" => "Register = min of a or b",
    "abs" => "Register = the absolute value of a",
    "log" => "Register = log(a)",
    "exp" => "Register = exp(a)",
    "rand" => "Register = a random value x with 0 &lt;= x &lt; 1",
    "yield" => "Pauses execution for 1 tick",
    "label" => "DEPRECATED - Use alias instead",
    "peek" => "Register = the value at the top of the stack",
    "push" => "Pushes the value of a to the stack at sp and increments sp",
    "pop" => "Register = the value at the top of the stack and decrements sp",
    "hcf" => "Halt and catch fire",
    "select" => "Register = b if a is non-zero, otherwise c",
    "sleep" => "Pauses execution on the IC for a seconds",
    "sin" => "Returns the sine of the specified angle (radians)",
    "cos" => "Returns the cosine of the specified angle (radians)",
    "tan" => "Returns the tan of the specified angle (radians) ",
    "asin" => "Returns the angle (radians) whos sine is the specified value",
    "acos" => "Returns the angle (radians) whos cosine is the specified value",
    "atan" => "Returns the angle (radians) whos tan is the specified value",
    "atan2" => "Returns the angle (radians) whose tangent is the quotient of two specified values: a (y) and b (x)",
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matching_instructions() {
        for instruction in INSTRUCTIONS.keys() {
            println!("Is {instruction} in INSTRUCTION_DOCS?");
            assert!(INSTRUCTION_DOCS.contains_key(instruction));
        }
        for instruction in INSTRUCTION_DOCS.keys() {
            println!("Is {instruction} in INSTRUCTIONS?");
            assert!(INSTRUCTIONS.contains_key(instruction));
        }
    }
}
