import subprocess
import json
import os

# Read tokens
with open(os.path.expanduser('~/jev-tokens.test'), 'r') as f:
    lines = f.read().splitlines()
    typesafe_token = lines[0].strip()
    openrouter_token = lines[1].strip()

cli_path = "/home/slsal/jev-cli/target/release/jev-cli"

tests = [
    {
        "name": "Noul - Typesafe",
        "provider": "typesafe.ai",
        "token": typesafe_token,
        "args": ["The sky is blue.", "Is this statement factually true?"]
    },
    {
        "name": "Choice - Typesafe",
        "provider": "typesafe.ai",
        "token": typesafe_token,
        "args": ["Apples are typically red or green.", "What fruit is being described?", "Apple", "Banana"]
    },
    {
        "name": "Score - Typesafe",
        "provider": "typesafe.ai",
        "token": typesafe_token,
        "args": ["I absolutely hated the movie.", "Score the sentiment", "--scale", "Very Negative", "Negative", "Neutral", "Positive", "Very Positive"]
    },
    {
        "name": "Noul - OpenRouter",
        "provider": "openrouter",
        "token": openrouter_token,
        "args": ["The sky is blue.", "Is this statement factually true?"]
    },
    {
        "name": "Choice - OpenRouter",
        "provider": "openrouter",
        "token": openrouter_token,
        "args": ["Apples are typically red or green.", "What fruit is being described?", "Apple", "Banana"]
    },
    {
        "name": "Score - OpenRouter",
        "provider": "openrouter",
        "token": openrouter_token,
        "args": ["I absolutely hated the movie.", "Score the sentiment", "--scale", "Very Negative", "Negative", "Neutral", "Positive", "Very Positive"]
    }
]

html_content = """
<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <title>Relatório de Testes de Integração - Jev CLI</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: #121212; color: #e0e0e0; padding: 20px; }
        h1 { color: #64ffda; border-bottom: 1px solid #333; padding-bottom: 10px; }
        .test-card { background-color: #1e1e1e; border: 1px solid #333; border-radius: 8px; padding: 20px; margin-bottom: 20px; }
        .test-title { font-size: 1.2em; font-weight: bold; margin-bottom: 10px; display: flex; align-items: center; }
        .status-success { color: #4caf50; margin-left: auto; font-size: 0.9em; border: 1px solid #4caf50; padding: 3px 8px; border-radius: 4px; }
        .status-error { color: #f44336; margin-left: auto; font-size: 0.9em; border: 1px solid #f44336; padding: 3px 8px; border-radius: 4px; }
        .cmd { background-color: #000; padding: 10px; border-radius: 5px; color: #ffb74d; font-family: monospace; font-size: 0.9em; margin-bottom: 15px; }
        .output { background-color: #000; padding: 15px; border-radius: 5px; color: #a5d6ff; font-family: monospace; font-size: 0.9em; white-space: pre-wrap; overflow-x: auto; }
    </style>
</head>
<body>
    <h1>Relatório de Testes de Integração (API Real)</h1>
    <p>Execução completa das 6 permutações contra Typesafe e OpenRouter usando a build mais recente da CLI.</p>
"""

for test in tests:
    cmd = [cli_path, "--token", test['token'], "--provider", test['provider']] + test['args']
    
    # Run the command
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    status_class = "status-success" if result.returncode == 0 else "status-error"
    status_text = "SUCESSO" if result.returncode == 0 else f"ERRO (Code {result.returncode})"
    
    # Hide the token in the command display
    safe_cmd_str = f"jev --token ****** --provider {test['provider']} " + " ".join([f'"{a}"' if ' ' in a else a for a in test['args']])
    
    output = result.stdout if result.returncode == 0 else result.stderr
    if not output:
        output = result.stdout + result.stderr

    html_content += f"""
    <div class="test-card">
        <div class="test-title">{test['name']} <span class="{status_class}">{status_text}</span></div>
        <div class="cmd">$ {safe_cmd_str}</div>
        <div class="output">{output.strip()}</div>
    </div>
    """

html_content += """
</body>
</html>
"""

with open('/home/slsal/jev-cli/report.html', 'w') as f:
    f.write(html_content)

print("Relatório gerado em /home/slsal/jev-cli/report.html")
