import random
import os

FILENAME = "comprehensive_test.bib"
TARGET_SIZE_MB = 2.5  # Target size in MB

# ---------------------------------------------------------------------------
# DATA MODELS & TEMPLATES (Derived from biblatex database guide)
# ---------------------------------------------------------------------------

# Standard BibLaTeX Entry Types
ENTRY_TYPES = [
    "article", "book", "mvbook", "inbook", "bookinbook", "suppbook",
    "booklet", "collection", "mvcollection", "incollection", "suppcollection",
    "dataset", "manual", "misc", "online", "patent", "periodical",
    "suppperiodical", "proceedings", "mvproceedings", "inproceedings",
    "reference", "mvreference", "inreference", "report", "set", "software",
    "thesis", "unpublished", "xdata", "customa"
]

# Latin-1 and UTF-8 characters for stress testing encoding
SPECIAL_CHARS = [
    r"{\"O}zge", r"{\c{C}}etinkaya", "Büyük", "Müller", "Ångström",
    "Æneid", "Dvořák", "Jørgen", "Płonka", "Garçon", "Ñuñez",
    "Sørensen", "Strauß", "voilà", "naïve", "東京"
]

# Standard fields to rotate through
FIELDS = [
    "subtitle", "titleaddon", "language", "edition", "volumes", "series",
    "number", "note", "publisher", "location", "isbn", "eid", "chapter",
    "pages", "pagetotal", "addendum", "pubstate", "doi", "eprint",
    "eprintclass", "eprinttype", "url", "urldate", "version", "abstract",
    "library", "nameaddon", "shortauthor", "shorteditor", "label", "shorthand",
    "shorthandintro", "shortjournal", "shortseries", "shorttitle", "indexsorttitle",
    "keywords", "options", "presort", "related", "relatedoptions", "relatedtype",
    "relatedstring", "sortkey", "sortname", "sortshorthand", "sorttitle", "sortyear"
]

LOREM_TITLES = [
    "The Theory of Everything", "Notes on {BibLaTeX}", "A Comprehensive Guide",
    "Rust Parsing Internals", "Memory Safety in {C}++", "Quantum Computing",
    "The {LaTeX} Companion", "History of Typography", "Algorithmic Complexity",
    "Deriving Atomic Charges", "Effect of Immobilization", "Higher-Dimensional Algebra"
]

# ---------------------------------------------------------------------------
# STATIC CONTENT (The "Gold Standard" parsing tests)
# ---------------------------------------------------------------------------

PREAMBLE_AND_STRINGS = """% Encoding: UTF-8
@preamble{ "Maintained by the Rust Compliance Suite" }
@string{ anch-ie = {Angew.~Chem. Int.~Ed.} }
@string{ jomch   = {J.~Organomet. Chem.} }
@string{ cup     = {Cambridge University Press} }
@string{ dt      = {Deutsche Thompson} }

"""

# These entries test specific parsing mechanics (concatenation, xdata, aliases, crossref)
EDGE_CASE_ENTRIES = [
    """
@article{concatenation_test,
  author       = "Knuth, Donald" # " and " # "Lamport, Leslie",
  title        = {Concatenated Strings Test},
  journaltitle = jomch,
  year         = 1999,
  month        = jan # "~" # 15
}
""",
    """
@xdata{macmillan:pub,
  publisher = {Macmillan},
  location  = {New York and London},
}

@book{xdata_consumer,
  author    = {Author, X.},
  title     = {XData Test},
  date      = {2020},
  xdata     = {macmillan:pub},
}
""",
    """
@inproceedings{crossref_child,
  author    = {Child, Entry},
  title     = {I Inherit Data},
  pages     = {10--20},
  crossref  = {crossref_parent},
}

@proceedings{crossref_parent,
  editor    = {Parent, Entry},
  title     = {Proceedings of the Parent},
  year      = {2023},
  publisher = cup,
}
""",
    """
@set{dynamic_set,
  entryset = {concatenation_test, xdata_consumer},
  annotation = {A set of dynamic entries}
}
""",
    """
@patent{complex_names,
  author = {Almendro, Jos{\'e} L. and Mart{\'i}n, Jacinto and {Corporate Holder, Inc.}},
  title  = {Elektromagnetisches Signalhorn},
  holder = {{Robert Bosch GmbH} and {Daimler Chrysler AG}},
  location = {countryfr and countryuk and countryde},
  date   = {1998-01-01/1998-02-01},
}
""",
    """
@online{uri_test,
  title = {URL Encoding Test},
  url   = {https://example.com/foo?bar=baz&qux=1%202},
  urldate = {2024-01-01T12:00:00},
  abstract = {Test % comment inside field}
}
""",
    """
@article{whitespace_stress,
  title = {  Weird
      Whitespace
  Test  },
  year = 2024,
  volume = { 10 },
  pages = " 100 -- 200 ",
}
""",
    """
@book{date_parsing_stress,
  title = {ISO 8601-2 Stress Test},
  date = {1997/},
  origdate = {1997-02-28T12:00:00},
  eventdate = {199X},
  urldate = {2000-01-01/2000-01-05},
  author = {Time Traveler}
}
"""
]

# ---------------------------------------------------------------------------
# GENERATOR FUNCTIONS
# ---------------------------------------------------------------------------

def random_name():
    first = random.choice(["John", "Jane", "Alice", "Bob", "Clara", "Dirk", "Émile", "Jean-Luc"])
    last = random.choice(["Doe", "Smith", "O'Connor", "Von Neumann", "Müller", "Li", "Garcia"])
    if random.random() < 0.1:
        return f"{{{first} {last} Corp.}}" # Corporate author protected by braces
    if random.random() < 0.2:
        return f"{last}, {first}" # Last, First format
    return f"{first} {last}"

def random_person_list(count=1):
    names = [random_name() for _ in range(count)]
    return " and ".join(names)

def random_date():
    # Mix of ISO formats from the Guide (Table 3 & 4)
    fmt = random.choice(["YYYY", "YYYY-MM", "YYYY-MM-DD", "Range", "OpenRange", "Unspecified"])
    year = random.randint(1800, 2030)

    if fmt == "YYYY":
        return f"{year}"
    elif fmt == "YYYY-MM":
        return f"{year}-{random.randint(1,12):02d}"
    elif fmt == "YYYY-MM-DD":
        return f"{year}-{random.randint(1,12):02d}-{random.randint(1,28):02d}"
    elif fmt == "Range":
        return f"{year}/{year+random.randint(1,5)}"
    elif fmt == "OpenRange":
        return f"{year}/"
    else:
        # 199X style
        return f"{str(year)[:3]}X"

def generate_entry(index):
    entry_type = random.choice(ENTRY_TYPES)
    key = f"entry_{index}_{entry_type}"

    # Core fields
    content = []

    # Randomly switch between quote types for values: {} or "" or unquoted numbers
    def wrap(val, is_number=False):
        if is_number and random.random() > 0.8:
            return f"{val}" # Unquoted number
        if random.random() > 0.5:
            return f"{{{val}}}"
        else:
            return f'"{val}"'

    # Author/Editor
    if entry_type in ["periodical", "proceedings", "collection"]:
        content.append(f"  editor = {wrap(random_person_list(random.randint(1, 3)))}")
    else:
        content.append(f"  author = {wrap(random_person_list(random.randint(1, 4)))}")

    # Title with random special chars
    title_text = random.choice(LOREM_TITLES)
    if random.random() > 0.7:
        title_text += f" with {random.choice(SPECIAL_CHARS)}"
    content.append(f"  title = {wrap(title_text)}")

    # Date
    content.append(f"  date = {wrap(random_date())}")

    # Random optional fields
    for _ in range(random.randint(2, 6)):
        field = random.choice(FIELDS)
        if field == "pages":
            start = random.randint(1, 900)
            val = f"{start}--{start+random.randint(5, 30)}"
            content.append(f"  {field} = {wrap(val)}")
        elif field == "isbn":
            val = f"978-{random.randint(100000000, 999999999)}"
            content.append(f"  {field} = {wrap(val)}")
        elif field == "url":
            val = "http://example.com/" + "".join(random.choices("abcdef123456", k=10))
            content.append(f"  {field} = {wrap(val)}")
        elif field == "doi":
            val = f"10.1000/{random.randint(1000,9999)}"
            content.append(f"  {field} = {wrap(val)}")
        elif field == "options":
            val = "useprefix=true, sortcites=false"
            content.append(f"  {field} = {wrap(val)}")
        elif field == "keywords":
            val = "primary, secondary, rust-test"
            content.append(f"  {field} = {wrap(val)}")
        elif field == "ids":
            val = f"alias_{key}"
            content.append(f"  {field} = {wrap(val)}")
        elif field == "gender":
            val = random.choice(["sm", "sf", "pp", "sn", "pf", "pm", "pn"])
            content.append(f"  {field} = {wrap(val)}")
        elif field == "volume":
            # Test unquoted numbers
            val = random.randint(1, 100)
            content.append(f"  {field} = {wrap(val, is_number=True)}")
        else:
            val = f"Random Value {random.randint(1, 1000)}"
            content.append(f"  {field} = {wrap(val)}")

    # Construct block
    # Randomly add spaces/indentation variance to test parser robustness
    indent = " " * random.randint(1, 4)
    entry_body = f",\n{indent}".join(content)

    return f"@{entry_type}{{{key},\n{indent}{entry_body}\n}}\n"

# ---------------------------------------------------------------------------
# MAIN EXECUTION
# ---------------------------------------------------------------------------

def create_bib_file():
    print(f"Generating {FILENAME} with target size {TARGET_SIZE_MB}MB...")

    with open(FILENAME, "w", encoding="utf-8") as f:
        # 1. Write Header & Preamble
        f.write(PREAMBLE_AND_STRINGS)

        # 2. Write Gold Standard Edge Cases
        f.write("\n% --- EDGE CASE ENTRIES (FROM GUIDE) ---\n")
        for entry in EDGE_CASE_ENTRIES:
            f.write(entry)

        # 3. Bulk Generation Loop
        f.write("\n% --- BULK GENERATED ENTRIES ---\n")

        count = 0
        current_size = 0
        target_bytes = TARGET_SIZE_MB * 1024 * 1024

        while current_size < target_bytes:
            # Buffer writes to avoid IO overhead
            buffer = []
            for _ in range(100): # Generate batches of 100
                buffer.append(generate_entry(count))
                count += 1

            block = "\n".join(buffer)
            f.write(block)
            f.flush() # Ensure size check is accurate
            current_size = os.path.getsize(FILENAME)

            if count % 1000 == 0:
                print(f"Generated {count} entries... ({current_size / 1024 / 1024:.2f} MB)")

    print(f"Done! Created {FILENAME} with {count} entries. Size: {current_size / 1024 / 1024:.2f} MB")

if __name__ == "__main__":
    create_bib_file()
