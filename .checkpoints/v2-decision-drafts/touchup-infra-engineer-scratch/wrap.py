import textwrap, sys
def wrap(text, first, rest, width=120):
    return '\n'.join(textwrap.wrap(' '.join(text.split()), width=width, initial_indent=first,
                     subsequent_indent=rest, break_on_hyphens=False, break_long_words=False))
