# Example Python plugin for Cyl language
# Save this as plugins/example_plugin.py

class LanguagePlugin:
    def register_syntax(self):
        return ["custom_syntax"]

    def register_types(self):
        return ["CustomType"]

    def register_functions(self):
        return {"custom_func": lambda args: f"CustomFunc called with {args}"}

    def eval_hook(self, expr):
        if expr == "custom_syntax":
            return "Handled by ExamplePlugin"
        return None
