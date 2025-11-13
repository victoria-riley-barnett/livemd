# livemd

A Markdown streaming tool for terminals. Streams Markdown content as it's generated, with basic formatting powered by [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) and [termimad](https://github.com/Canop/termimad).

## Important Notes

⚠️ **This is a tool with limitations:**
- Not all Markdown features are supported, especially extended syntax
- Compatibility has not been extensively tested.
- Streaming may not work perfectly with all content, especially complex layouts: formatting may break, might flush imperfectly if it's not seeing the right boundaries.
