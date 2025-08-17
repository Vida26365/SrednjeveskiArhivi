from pydantic import BaseModel, Field


class Document(BaseModel):
    summary: str = Field(
        title="Summary",
        description="A summary of the text, commonly included at the beginning of the document.",
    )

    metadata: str = Field(
        title="Metadata",
        description="Metadata about the document, such as date, location, sources, typist, and seal status.",
    )

    content: str = Field(
        title="Content",
        description="The main content of the document.",
    )


class Metadata(BaseModel):
    date: str | None = Field(
        title="Date",
        description="The date from the document header. Must be in the format `YYYY-MM-DD`.",
    )

    location: str | None = Field(
        title="Location",
        description="The location from the document header. Must be exactly the same as in the source, without any punctuation.",
    )

    languages: list[str] = Field(
        title="Languages",
        description="A list of languages used in the document content. Must be a list of two-letter language codes.",
    )

    keywords: list[str] = Field(
        title="Keywords",
        description="A list of up to 5 main keywords in Slovene from the document content.",
    )
