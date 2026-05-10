/*
 * Generated type guards for "displayTemplate.ts".
 * WARNING: Do not manually change this file.
 */
import type { DisplayTemplate } from './displayTemplate';

export function isDisplayTemplate(obj: unknown): obj is DisplayTemplate {
    const typedObj = obj as DisplayTemplate;
    return (
        ((typedObj !== null && typeof typedObj === 'object') || typeof typedObj === 'function') &&
        Array.isArray(typedObj['stats']) &&
        typedObj['stats'].every(
            (e: any) =>
                ((e !== null && typeof e === 'object') || typeof e === 'function') &&
                typeof e['name'] === 'string' &&
                (e['datatype'] === 'string' ||
                    e['datatype'] === 'int' ||
                    e['datatype'] === 'float' ||
                    e['datatype'] === 'bool') &&
                typeof e['default'] === 'string' &&
                (e['visibility'] === 'public' || e['visibility'] === 'private') &&
                typeof e['pos'] === 'string'
        ) &&
        Array.isArray(typedObj['player_stats']) &&
        typedObj['player_stats'].every(
            (e: any) =>
                ((e !== null && typeof e === 'object') || typeof e === 'function') &&
                typeof e['name'] === 'string' &&
                (e['datatype'] === 'string' ||
                    e['datatype'] === 'int' ||
                    e['datatype'] === 'float' ||
                    e['datatype'] === 'bool') &&
                typeof e['default'] === 'string' &&
                (e['visibility'] === 'public' ||
                    e['visibility'] === 'private' ||
                    e['visibility'] === 'protected') &&
                typeof e['pos'] === 'string'
        ) &&
        Array.isArray(typedObj['actions']) &&
        typedObj['actions'].every(
            (e: any) =>
                ((e !== null && typeof e === 'object') || typeof e === 'function') &&
                typeof e['name'] === 'string' &&
                Array.isArray(e['parameters']) &&
                e['parameters'].every((e: any) => typeof e === 'string') &&
                Array.isArray(e['execution_triggers']) &&
                e['execution_triggers'].every((e: any) => typeof e === 'string') &&
                (e['visibility'] === 'public' || e['visibility'] === 'private') &&
                typeof e['source_code'] === 'string' &&
                typeof e['pos'] === 'string'
        )
    );
}
