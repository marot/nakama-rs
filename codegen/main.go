// Copyright 2020 The Nakama Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package main

import (
	"bufio"
	"encoding/json"
	"flag"
	"fmt"
	"io/ioutil"
	"os"
	"strings"
	"text/template"
	"unicode"
)

const codeTemplate string = `/* Code generated by codegen/main.go. DO NOT EDIT. */

use std::collections::HashMap;

use nanoserde::{DeJson,SerJson};
use urlencoding::encode;


#[derive(Debug, Clone)]
pub enum Authentication {
  Basic {
    username: String,
    password: String
  },
  Bearer {
    token: String
  }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Method {
    Post, Get, Put, Delete
}

#[derive(Debug, Clone)]
pub struct RestRequest<Response> {
  pub authentication: Authentication,
  pub urlpath: String,
  pub query_params: String,
  pub body: String,
  pub method: Method,
  _marker: std::marker::PhantomData<Response>
}

{{- range $defname, $definition := .Definitions }}
{{- $classname := $defname | title }}

{{- if isRefToEnum $defname }}

/// {{ $definition.Description | stripNewlines }}
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum {{ $classname }} {
    {{- range $idx, $enum := $definition.Enum }}
    /// {{ (index (splitEnumDescription $definition.Description) $idx) }}
    {{ $enum }} = {{ $idx }},
    {{- end }}
}

{{- else }}

/// {{ $definition.Description | stripNewlines }}
#[derive(Debug, DeJson, SerJson, Default, Clone)]
#[nserde(default)]
pub struct {{ $classname }} {
    {{- range $propname, $property := $definition.Properties }}
    {{- $fieldname := $propname | camelToSnake }}
    {{- $attrDataName := $propname | camelToSnake }}

    {{- if eq $property.Type "integer" }}
    pub {{ $fieldname }}: i32,

    {{- else if eq $property.Type "number" }}
    pub {{ $fieldname }}: f32,

    {{- else if eq $property.Type "boolean" }}
    pub {{ $fieldname }}: bool,

    {{- else if eq $property.Type "string" }}
    {{- if contains $property.Description "optional" }}
    pub {{ $fieldname }}: Option<String>,
    {{- else }}
    pub {{ $fieldname }}: String,
    {{- end }}

    {{- else if eq $property.Type "array" }}
	{{- if eq $property.Items.Type "string" }}
    pub {{ $fieldname }}: Vec<String>,
	{{- else if eq $property.Items.Type "integer" }}
    pub {{ $fieldname }}: Vec<i32>,
	{{- else if eq $property.Items.Type "number" }}
    pub {{ $fieldname }}: Vec<f32>,
	{{- else if eq $property.Items.Type "boolean" }}
    pub {{ $fieldname }}: Vec<bool>,
	{{- else}}
    pub {{ $fieldname }}: Vec<{{ $property.Items.Ref | cleanRef }}>,
	{{- end }}
    {{- else if eq $property.Type "object"}}
	{{- if eq $property.AdditionalProperties.Type "string"}}
    pub {{ $fieldname }}: HashMap<String, String>,
	{{- else if eq $property.Items.Type "integer"}}
    pub {{ $fieldname }}: HashMap<String, i32>,
       {{- else if eq $property.Items.Type "number"}}
    pub {{ $fieldname }}: HashMap<String, f32>,
	{{- else if eq $property.Items.Type "boolean"}}
    pub {{ $fieldname }}: HashMap<string, bool>,
	{{- else}}
    pub {{ $fieldname }}: HashMap<string, {{$property.AdditionalProperties | cleanRef}}>,
	{{- end}}
    {{- else }}
    pub {{ $fieldname }}: {{ $property.Ref | cleanRef }},
    {{- end }}
    {{- end }}
}

{{- end }}
{{- end }}

{{- range $url, $path := .Paths }}
{{- range $method, $operation := $path}}
/// {{ $operation.Summary | stripNewlines }}
pub fn {{ $operation.OperationId | stripOperationPrefix | pascalToCamel | camelToSnake	 }}(
{{- if $operation.Security }}
{{- with (index $operation.Security 0) }}
    {{- range $key, $value := . }}
	{{- if eq $key "BasicAuth" }}
    basic_auth_username: &str,
    basic_auth_password: &str,
	{{- else if eq $key "HttpKeyAuth" }}
    bearer_token: &str,
	{{- end }}
    {{- end }}
{{- end }}
{{- else }}
    bearer_token: &str,
{{- end }}
{{- range $parameter := $operation.Parameters }}
{{- $argname := $parameter.Name | camelToSnake }}
{{- if eq $parameter.In "path" }}
    {{- if eq $parameter.Type "string" }}
    {{ $argname }}:{{ " " }}{{- if not $parameter.Required }}Option<{{- end }}&str{{- if not $parameter.Required }}>{{- end }}
    {{- else }}
    {{ $argname }}:{{ " " }}{{- if not $parameter.Required }}Option<{{- end }}{{ $parameter.Type }}{{- if not $parameter.Required }}>{{- end }}
    {{- end }}
{{- else if eq $parameter.In "body" }}
    {{- if eq $parameter.Schema.Type "string" }}
    {{ $argname }}:{{ " " }}{{- if not $parameter.Required }}Option<{{- end }}&str{{- if not $parameter.Required }}>{{- end }}
    {{- else }}
    {{ $argname }}:{{ " " }}{{- if not $parameter.Required }}Option<{{- end }}{{ $parameter.Schema.Ref | cleanRef }}{{- if not $parameter.Required }}>{{- end }}
    {{- end }}
{{- else if eq $parameter.Type "array"}}
    {{- if eq $parameter.Items.Type "string" }}
    {{ $argname }}: &[String]
    {{- else }}
    {{ $argname }}: &[{{ $parameter.Items.Type }}]
    {{- end }}
{{- else if eq $parameter.Type "object"}}
    {{- if eq $parameter.AdditionalProperties.Type "string"}}
IDictionary<string, string> {{ $parameter.Name }}
    {{- else if eq $parameter.Items.Type "integer"}}
IDictionary<string, int> {{ $parameter.Name }}
    {{- else if eq $parameter.Items.Type "boolean"}}
IDictionary<string, int> {{ $parameter.Name }}
    {{- else}}
IDictionary<string, {{ $parameter.Items.Type }}> {{ $parameter.Name }}
    {{- end}}
{{- else if eq $parameter.Type "integer" }}
    {{ $argname }}: Option<i32>
{{- else if eq $parameter.Type "boolean" }}
    {{ $argname }}: Option<bool>
{{- else if eq $parameter.Type "string" }}
    {{ $argname }}: Option<&str>
{{- else }}
    {{ $argname }}: Option<{{ $parameter.Type }}>
{{- end }},
{{- end }}
{{- if $operation.Responses.Ok.Schema.Ref }}
) -> RestRequest<{{ $operation.Responses.Ok.Schema.Ref | cleanRef }}> {
{{- else }}
) -> RestRequest<()> {
{{- end }}
    #[allow(unused_mut)]
    let mut urlpath = "{{- $url }}".to_string();

    {{- range $parameter := $operation.Parameters }}
    {{- $argname := $parameter.Name | camelToSnake }}
    {{- if eq $parameter.In "path" }}
    urlpath = urlpath.replace("{{- print "{" $parameter.Name "}"}}",{{" "}} {{- $argname }});
    {{- end }}
    {{- end }}

    #[allow(unused_mut)]
    let mut query_params = String::new();

{{- range $parameter := $operation.Parameters }}
    {{- $argname := $parameter.Name | camelToSnake }}
{{- if eq $parameter.In "query"}}
    {{- if eq $parameter.Type "integer" }}
if let Some(param) = {{ $argname }} {
    query_params.push_str(&format!("{{- $argname }}={}&", param));
}
    {{- else if eq $parameter.Type "string" }}
if let Some(param) = {{ $argname }} {
    query_params.push_str(&format!("{{- $argname }}={}&", encode(param)));
}
    {{- else if eq $parameter.Type "boolean" }}
if let Some(param) = {{ $argname }} {
    query_params.push_str(&format!("{{- $argname }}={:?}&", param));
}
    {{- else if eq $parameter.Type "array" }}
for elem in {{ $argname }}
{
    query_params.push_str(&format!("{{- $argname }}={}&", encode(elem)));
}
    {{- else }}
{{ $parameter }} // ERROR
    {{- end }}
{{- end }}
{{- end }}

    let authentication = {{- if $operation.Security }}
{{- with (index $operation.Security 0) }}
    {{- range $key, $value := . }}
	{{- if eq $key "BasicAuth" }}
Authentication::Basic {
	username: basic_auth_username.to_owned(),
	password: basic_auth_password.to_owned()
    };
	{{- else if eq $key "HttpKeyAuth" }}
    Authentication::Bearer {
	token: bearer_token.to_owned()
    };
	{{- end }}
    {{- end }}
{{- end }}
{{- else }}
    Authentication::Bearer {
	token: bearer_token.to_owned()
    };
{{- end }}

    {{- $hasBody := false }}
    {{- range $parameter := $operation.Parameters }}
    {{- if eq $parameter.In "body" }}
    {{- $hasBody = true }}
    {{- if eq $parameter.Schema.Type "string" }}
    let body_json = {{ $parameter.Name }}.to_string();
    {{- else }}
    let body_json = {{ $parameter.Name }}.serialize_json();
    {{- end }}
    {{- end }}
    {{- end }}
    {{ if eq $hasBody false }}
    let body_json = String::new();
    {{- end }}

    let method = Method::{{- $method | camelToPascal }};

    RestRequest {
       authentication,
       urlpath,
       query_params,
       body: body_json,
       method,
       _marker: std::marker::PhantomData
    }
}

{{- end }}
{{- end }}
`

func convertRefToClassName(input string) (className string) {
	cleanRef := strings.TrimPrefix(input, "#/definitions/")
	className = strings.Title(cleanRef)
	return
}

// camelToSnake converts a camel or Pascal case string into snake case.
func camelToSnake(input string) (output string) {
	for k, v := range input {
		if unicode.IsUpper(v) {
			formatString := "%c"

			if k != 0 {
				formatString = "_" + formatString
			}

			output += fmt.Sprintf(formatString, unicode.ToLower(v))
		} else {
			output += string(v)
		}
	}

	return
}

func snakeToCamel(input string) (snakeToCamel string) {
	isToUpper := false
	for k, v := range input {
		if k == 0 {
			snakeToCamel = strings.ToLower(string(input[0]))
		} else {
			if isToUpper {
				snakeToCamel += strings.ToUpper(string(v))
				isToUpper = false
			} else {
				if v == '_' {
					isToUpper = true
				} else {
					snakeToCamel += string(v)
				}
			}
		}

	}
	return
}

func snakeToPascal(input string) (output string) {
	isToUpper := false
	for k, v := range input {
		if k == 0 {
			output = strings.ToUpper(string(input[0]))
		} else {
			if isToUpper {
				output += strings.ToUpper(string(v))
				isToUpper = false
			} else {
				if v == '_' {
					isToUpper = true
				} else {
					output += string(v)
				}
			}
		}
	}
	return
}

func isPropertyEnum(string) (output string) {
	return
}

// pascalToCamel converts a Pascal case string to a camel case string.
func pascalToCamel(input string) (camelCase string) {
	if input == "" {
		return ""
	}

	camelCase = strings.ToLower(string(input[0]))
	camelCase += string(input[1:])
	return camelCase
}

func splitEnumDescription(description string) (output []string) {
	return strings.Split(description, "\n")
}

func stripNewlines(input string) string {
	return strings.Replace(input, "\n", " ", -1)
}

func stripOperationPrefix(input string) string {
	return strings.Replace(input, "Nakama_", "", 1)
}

func descriptionOrTitle(description string, title string) string {
	if description != "" {
		return description
	}

	return title
}

// camelToPascal converts a string from camel case to Pascal case.
func camelToPascal(camelCase string) (pascalCase string) {

	if len(camelCase) <= 0 {
		return ""
	}

	pascalCase = strings.ToUpper(string(camelCase[0])) + camelCase[1:]
	return
}

func main() {
	// Argument flags
	var output = flag.String("output", "", "The output for generated code.")
	flag.Parse()

	inputs := flag.Args()
	if len(inputs) < 1 {
		fmt.Printf("No input file found: %s\n\n", inputs)
		fmt.Println("openapi-gen [flags] inputs...")
		flag.PrintDefaults()
		return
	}

	inputFile := inputs[0]
	content, err := ioutil.ReadFile(inputFile)
	if err != nil {
		fmt.Printf("Unable to read file: %s\n", err)
		return
	}

	var subnamespace (string) = ""

	if len(inputs) > 1 {
		if len(inputs[1]) <= 0 {
			fmt.Println("Empty Sub-Namespace provided.")
			return
		}

		subnamespace = inputs[1]
	}

	var schema struct {
		SubNamespace string
		Paths        map[string]map[string]struct {
			Summary     string
			OperationId string
			Responses   struct {
				Ok struct {
					Schema struct {
						Ref string `json:"$ref"`
					}
				} `json:"200"`
			}
			Parameters []struct {
				Name     string
				In       string
				Required bool
				Type     string   // used with primitives
				Items    struct { // used with type "array"
					Type string
				}
				Schema struct { // used with http body
					Type string
					Ref  string `json:"$ref"`
				}
				Format string // used with type "boolean"
			}
			Security []map[string][]struct {
			}
		}
		Definitions map[string]struct {
			Properties map[string]struct {
				Type  string
				Ref   string   `json:"$ref"` // used with object
				Items struct { // used with type "array"
					Type string
					Ref  string `json:"$ref"`
				}
				AdditionalProperties struct {
					Type string // used with type "map"
				}
				Format      string // used with type "boolean"
				Description string
				Title       string // used by enums
			}
			Enum        []string
			Description string
			// used only by enums
			Title string
		}
	}

	schema.SubNamespace = subnamespace

	if err := json.Unmarshal(content, &schema); err != nil {
		fmt.Printf("Unable to decode input file %s : %s\n", inputFile, err)
		return
	}

	fmap := template.FuncMap{
		"snakeToCamel" :        snakeToCamel,
		"camelToSnake" :        camelToSnake,
		"stripNewlines":        stripNewlines,
		"title":                strings.Title,
		"uppercase":            strings.ToUpper,
		"stripOperationPrefix": stripOperationPrefix,
		"cleanRef":     convertRefToClassName,
		"contains": func(a string, b string) bool {
		    return strings.Contains(a, b)
		},
		"isRefToEnum": func(ref string) bool {
			// swagger schema definition keys have inconsistent casing
			var camelOk bool
			var pascalOk bool
			var enums []string

			asCamel := pascalToCamel(ref)
			if _, camelOk = schema.Definitions[asCamel]; camelOk {
				enums = schema.Definitions[asCamel].Enum
			}

			asPascal := camelToPascal(ref)
			if _, pascalOk = schema.Definitions[asPascal]; pascalOk {
				enums = schema.Definitions[asPascal].Enum
			}

			if !pascalOk && !camelOk {
				fmt.Printf("no definition found: %v", ref)
				return false
			}

			return len(enums) > 0
		},
		"pascalToCamel":        pascalToCamel,
		"snakeToPascal":        snakeToPascal,
		"camelToPascal":        camelToPascal,
		"splitEnumDescription": splitEnumDescription,
		"descriptionOrTitle":   descriptionOrTitle,
	}

	tmpl, err := template.New(inputFile).Funcs(fmap).Parse(codeTemplate)
	if err != nil {
		fmt.Printf("Template parse error: %s\n", err)
		return
	}

	if len(*output) < 1 {
		tmpl.Execute(os.Stdout, schema)
		return
	}

	f, err := os.Create(*output)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	writer := bufio.NewWriter(f)
	tmpl.Execute(writer, schema)
	writer.Flush()
}
