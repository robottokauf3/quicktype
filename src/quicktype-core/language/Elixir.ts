import { TargetLanguage } from "../TargetLanguage";
import { Option, OptionValues, getOptionValues } from "../RendererOptions";
import { ConvenienceRenderer } from "../ConvenienceRenderer";
import { RenderContext } from "../Renderer";
import { Namer, funPrefixNamer } from "../Naming";
import { ObjectType, ClassProperty } from "../Type";

export const elixirOptions = {};

// Language Class
export class ElixirTargetLanguage extends TargetLanguage {
    constructor() {
        super("Elixir", ["elixir"], "ex");
    }
    getOptions(): Option<any>[] {
        return [];
    }

    makeRenderer(renderContext: RenderContext, untypedOptionValues: { [name: string]: any }): ElixirRenderer {
        return new ElixirRenderer(this, renderContext, getOptionValues(elixirOptions, untypedOptionValues));
    }
}

// Helper functions

const typeNamingFunction = funPrefixNamer("types", elixirNameStyle);

function elixirNameStyle(original: string): string {
    return original;
}

// Renderer Class
export class ElixirRenderer extends ConvenienceRenderer {
    constructor(
        targetLanguage: TargetLanguage,
        renderContext: RenderContext,
        private readonly _elixirOptions: OptionValues<typeof elixirOptions>
    ) {
        super(targetLanguage, renderContext);
    }

    protected makeNamedTypeNamer(): Namer {
        // WIP
        return typeNamingFunction;
    }

    protected namerForObjectProperty(o: ObjectType, p: ClassProperty): Namer | null {
        // TODO
        return null;
    }

    protected makeUnionMemberNamer(): Namer | null {
        // TODO
        return null;
    }
    protected makeEnumCaseNamer(): Namer | null {
        // TODO
        return null;
    }
    protected emitSourceStructure(): void {
        this.emitLine("// To parse and unparse this JSON data, add this code to your project and do:");
    }

}