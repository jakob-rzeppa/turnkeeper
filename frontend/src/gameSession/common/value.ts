import { err, ok, Result } from "neverthrow";

export const convertValueToStringValue = (type: "int" | "float" | "bool" | "string", newVal: string): Result<string, string> => {
    if (type === 'int') {
        const intVal = parseInt(newVal, 10);
        if (isNaN(intVal)) return err(`Invalid integer: "${newVal}"`);
        return ok("int(" + intVal + ")");
    }

    if (type === 'float') {
        const floatVal = parseFloat(newVal);
        if (isNaN(floatVal)) return err(`Invalid float: "${newVal}"`);
        return ok('float(' + floatVal + ')');
    }
    
    if (type === 'bool') {
        const lower = newVal.toLowerCase();
        if (!['true', 'false', '0', '1', 'yes', 'no'].includes(lower)) {
            return err(`Invalid boolean: "${newVal}". Use true/false, yes/no, or 0/1`);
        }
        return ok("bool(" + ['true', '1', 'yes'].includes(lower) + ")");
    }
    
    return ok("string(" + newVal + ")");
};

export const convertStringValueToValue = (stringVal: string): string | boolean => {
    if (stringVal.startsWith("int(") && stringVal.endsWith(")")) {
        return stringVal.slice(4, -1);
    }
    if (stringVal.startsWith("float(") && stringVal.endsWith(")")) {
        return stringVal.slice(6, -1);
    }
    if (stringVal.startsWith("bool(") && stringVal.endsWith(")")) {
        return stringVal.slice(5, -1) === "true";
    }
    if (stringVal.startsWith("string(") && stringVal.endsWith(")")) {
        return stringVal.slice(7, -1);
    }
    throw new Error(`Invalid string value format: "${stringVal}"`);
};