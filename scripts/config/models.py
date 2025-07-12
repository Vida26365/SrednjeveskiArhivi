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
