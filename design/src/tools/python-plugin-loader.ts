// Loads Python plugins and returns their info (ESM/async, Node.js only)
export async function loadPythonPlugins() {
    if (typeof process === 'undefined' || !process.versions?.node) {
        return { syntax: [], types: [], functions: [] };
    }
    const { spawnSync } = await import('child_process');
    const fs = (await import('fs')).default;
    const path = (await import('path')).default;
    const { fileURLToPath } = await import('url');
    const __dirname = path.dirname(fileURLToPath(import.meta.url));
    const pluginsDir = path.join(__dirname, '../../plugins');
    let syntax: string[] = [];
    let types: string[] = [];
    let functions: any[] = [];
    if (fs.existsSync(pluginsDir)) {
        for (const file of fs.readdirSync(pluginsDir)) {
            if (file.endsWith('.py')) {
                const pyCode = `import sys; sys.path.insert(0, '${pluginsDir}'); from ${file.replace('.py','')} import LanguagePlugin; p=LanguagePlugin(); import json; print(json.dumps({'syntax': p.register_syntax(), 'types': p.register_types()}))`;
                const result = spawnSync('python3', ['-c', pyCode], { encoding: 'utf8' });
                if (result.stdout) {
                    try {
                        const info = JSON.parse(result.stdout);
                        if (info.syntax) syntax.push(...info.syntax);
                        if (info.types) types.push(...info.types);
                    } catch {}
                }
            }
        }
    }
    return { syntax, types, functions };
}

