<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useGameEditor } from './useGameEditor';

const route = useRoute();

const gameEditor = useGameEditor();

interface ErrorLocation {
    line: number;
    column: number;
    message: string;
}

function parseErrors(
    errorData: readonly { readonly message: string; readonly pos: string }[] | string,
    fullCode: string = ''
): ErrorLocation[] {
    // Handle array of error objects from API response
    if (Array.isArray(errorData)) {
        return errorData
            .map(error => {
                // Handle EOF (End of File) case
                if (error.pos === 'EOF') {
                    const codeLines = fullCode.split('\n');
                    const lastLineIndex = codeLines.length - 1;
                    const lastLine = codeLines[lastLineIndex];
                    return {
                        line: lastLineIndex + 1,
                        column: lastLine!.length,
                        message: error.message,
                    };
                }

                // Parse pos string - format could be "line:col" or "line X, column Y"
                const match = error.pos.match(/(?:line\s+)?(\d+)(?:\D+(\d+))?/);
                if (match) {
                    return {
                        line: parseInt(match[1], 10) + 1, // Convert from 0-indexed to 1-indexed
                        column: match[2] ? parseInt(match[2], 10) : 1,
                        message: error.message,
                    };
                }
                return null;
            })
            .filter(e => e !== null) as ErrorLocation[];
    }
    // Fallback for string format
    if (typeof errorData === 'string') {
        const lines = errorData.split('\n');
        return lines
            .map(line => {
                const match = line.match(/line (\d+), column (\d+):\s*(.*)/i);
                if (match) {
                    return {
                        line: parseInt(match[1]!, 10),
                        column: parseInt(match[2]!, 10),
                        message: match[3],
                    };
                }
                return null;
            })
            .filter(e => e !== null) as ErrorLocation[];
    }
    return [];
}

function formatErrorContext(
    fullCode: string,
    error: ErrorLocation,
    contextLines: number = 5
): string {
    const codeLines = fullCode.split('\n');
    const errorLineIndex = error.line - 1;
    const startLine = Math.max(0, errorLineIndex - contextLines);
    const endLine = Math.min(codeLines.length, errorLineIndex + contextLines + 1);

    let result = '';
    for (let i = startLine; i < endLine; i++) {
        const lineNum = i + 1;
        const line = codeLines[i];
        const linePrefix = lineNum === error.line ? '> ' : '  ';
        result += `${linePrefix}${lineNum.toString().padStart(4)} | ${line}\n`;

        if (lineNum === error.line) {
            const caretPos = error.column - 1 + 10; // Adjust for prefix and line number
            result += ' '.repeat(caretPos) + '^\n';
            result += `        ${error.message}\n`;
        }
    }
    return result;
}
</script>

<template>
    <div v-if="gameEditor.isSaved.value !== true">
        <p class="alert alert-warning">You have unsaved changes.</p>
    </div>
    <div v-else-if="gameEditor.checkResult.value?.status === 'uninitialized'">
        <h2 class="text-xl font-bold mb-4">Uninitialized</h2>
        <button
            class="btn btn-primary"
            @click="gameEditor.checkSourceCode(route.params.id as string)"
        >
            Check Code
        </button>
    </div>
    <div v-else-if="gameEditor.checkResult.value?.status === 'loading'">
        <h2 class="text-xl font-bold mb-4">Checking...</h2>
    </div>
    <div v-else-if="gameEditor.checkResult.value?.status === 'error'">
        <h2 class="text-xl font-bold mb-4">Error</h2>
        <p>{{ gameEditor.checkResult.value.error }}</p>
        <button
            class="btn btn-primary"
            @click="gameEditor.checkSourceCode(route.params.id as string)"
        >
            Retry
        </button>
    </div>
    <div v-else-if="gameEditor.checkResult.value?.status === 'success'">
        <h2 class="text-xl font-bold mb-4">Check Result</h2>
        <div v-if="gameEditor.checkResult.value.data.is_valid">
            <p class="text-green-600">Code is valid!</p>
            <pre>
                {{ gameEditor.checkResult.value.data.output }}
            </pre>
        </div>
        <div v-else>
            <p class="text-error">Code is invalid.</p>
            <div
                v-for="(error, index) in parseErrors(
                    gameEditor.checkResult.value.data.errors,
                    gameEditor.sourceCode.value || ''
                )"
                :key="index"
                class="mt-4"
            >
                <pre class="border-l-4 border-error p-3 overflow-auto">{{
                    formatErrorContext(gameEditor.sourceCode.value || '', error)
                }}</pre>
            </div>
        </div>
    </div>
</template>
