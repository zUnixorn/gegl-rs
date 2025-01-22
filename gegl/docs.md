<!-- file * -->
<!-- fn apply_op -->
Apply the operation to buffer, overwritting the contents of buffer.
## `buffer`
the [`Buffer`][crate::Buffer] to apply onto
## `operation_name`
name of the operation to apply
<!-- fn apply_op_valist -->
Apply the operation to buffer, overwritting the contents of buffer.
## `buffer`
the [`Buffer`][crate::Buffer] to apply onto
## `operation_name`
name of the operation to apply
## `var_args`
the settings for the operation. Zero or more key/value pairs,
ended terminated with NULL.
<!-- fn calloc -->
allocated 0'd memory.
## `size`
size of items to allocate
## `n_memb`
number of members
<!-- fn create_chain -->
Create a node chain from an unparsed commandline string.
## `ops`
an argv style, NULL terminated array of arguments
## `op_start`
node to pass in as input of chain
## `op_end`
node to get processed data
## `time`
the time to use for interpolatino of keyframed values
## `rel_dim`
relative dimension to scale rel suffixed values by
## `path_root`
path in filesystem to use as relative root
<!-- fn filter_op -->
Apply the operation to source_buffer, returning the result in a new buffer.
## `source_buffer`
the source [`Buffer`][crate::Buffer] for the filter
## `operation_name`
name of the operation to apply

# Returns

the result of the filter
<!-- fn filter_op_valist -->
Apply the operation to source_buffer, returning the result in a new buffer.
## `source_buffer`
the source [`Buffer`][crate::Buffer] for the filter
## `operation_name`
name of the operation to apply
## `var_args`
the settings for the operation. Zero or more key/value pairs,
ended terminated with NULL.

# Returns

the result of the filter
<!-- fn graph_dump_outputs -->
Dump the bounds and format of each node in the graph to stdout.
## `node`
The final node of the graph
<!-- fn graph_dump_request -->
Dump the region that will be rendered for each node to fulfill
the request.
## `node`
The final node of the graph
## `roi`
The request rectangle
<!-- fn malloc -->
Allocates `n_bytes` of memory. If `n_bytes` is 0, returns NULL.

Returns a pointer to the allocated memory.
## `n_bytes`
the number of bytes to allocte.
<!-- fn memeq_zero -->
Checks if all the bytes of the memory block `ptr`, of size `size`,
are equal to zero.
## `ptr`
pointer to the memory block
## `size`
block size

# Returns

TRUE if all the bytes are equal to zero.
<!-- fn memset_pattern -->
Fill `dst_ptr` with `count` copies of the bytes in `src_ptr`.
## `dst_ptr`
pointer to copy to
## `src_ptr`
pointer to copy from
## `pattern_size`
the length of `src_ptr`
## `count`
number of copies
<!-- fn param_spec_curve -->
Creates a new [`glib::ParamSpec`][crate::glib::ParamSpec] instance specifying a `GeglCurve` property.
## `name`
canonical name of the property specified
## `nick`
nick name for the property specified
## `blurb`
description of the property specified
## `default_curve`
the default value for the property specified
## `flags`
flags for the property specified

# Returns

a newly created parameter specification
<!-- fn render_op -->
Apply the operation to source_buffer, writing the results to target_buffer.
## `source_buffer`
the source [`Buffer`][crate::Buffer] for the filter
## `target_buffer`
the source [`Buffer`][crate::Buffer] for the filter
## `operation_name`
name of the operation to apply
<!-- fn render_op_valist -->
Apply the operation to source_buffer, writing the results to target_buffer.
## `source_buffer`
the source [`Buffer`][crate::Buffer] for the filter
## `target_buffer`
the source [`Buffer`][crate::Buffer] for the filter
## `operation_name`
name of the operation to apply
## `var_args`
the settings for the operation. Zero or more key/value pairs,
ended terminated with NULL.
<!-- fn scratch_alloc -->
Allocates `size` bytes of scratch memory.

Returns a pointer to the allocated memory.
## `size`
the number of bytes to allocte.
<!-- fn scratch_alloc0 -->
Allocates `size` bytes of scratch memory, initialized to zero.

Returns a pointer to the allocated memory.
## `size`
the number of bytes to allocte.
<!-- fn serialize -->
## `start`
first node in chain to serialize
## `end`
last node in chain to serialize
## `basepath`
top-level absolute path to turn into relative root
## `serialize_flags`
anded together combination of:
GEGL_SERIALIZE_TRIM_DEFAULTS, GEGL_SERIALIZE_VERSION, GEGL_SERIALIZE_INDENT.
<!-- fn try_malloc -->
Allocates `n_bytes` of memory. If allocation fails, or if `n_bytes` is 0,
returns [`None`].

Returns a pointer to the allocated memory, or NULL.
## `n_bytes`
the number of bytes to allocte.
<!-- impl Buffer::fn linear_new_from_data -->
Creates a GeglBuffer backed by a linear memory buffer that already exists,
of the given `extent` in the specified `format`. babl_format ("R'G'B'A u8")
for instance to make a normal 8bit buffer.
## `data`
a pointer to a linear buffer in memory.
## `format`
the format of the data in memory
## `extent`
the dimensions (and upper left coordinates) of linear buffer.
## `rowstride`
the number of bytes between rowstarts in memory (or 0 to
 autodetect)
## `destroy_fn`
function to call to free data or NULL if memory should not be
 freed.
## `destroy_fn_data`
extra argument to be passed to void destroy(ptr, data) type
 function.

# Returns

a GeglBuffer that can be used as any other GeglBuffer.
<!-- impl Buffer::fn add_handler -->
Add a new tile handler in the existing chain of tile handler of a GeglBuffer.
## `handler`
a [`TileHandler`][crate::TileHandler]
<!-- impl Buffer::fn get -->
Fetch a rectangular linear buffer of pixel data from the GeglBuffer, the
data is converted to the desired BablFormat, if the BablFormat stored and
fetched is the same this amounts to a series of memcpy's aligned to demux
the tile structure into a linear buffer.
## `rect`
the coordinates we want to retrieve data from, and width/height of
destination buffer, if NULL equal to the extent of the buffer. The
coordinates and dimensions are after scale has been applied.
## `scale`
sampling scale, 1.0 = pixel for pixel 2.0 = magnify, 0.5 scale down.
## `format`
the BablFormat to store in the linear buffer `dest`.
## `dest`
the memory destination for a linear buffer for the pixels, the size needed
depends on the requested BablFormat.
## `rowstride`
rowstride in bytes, or GEGL_AUTO_ROWSTRIDE to compute the
rowstride based on the width and bytes per pixel for the specified format.
## `repeat_mode`
how requests outside the buffer extent are handled.
Valid values: GEGL_ABYSS_NONE (abyss pixels are zeroed), GEGL_ABYSS_WHITE
(abyss pixels are white), GEGL_ABYSS_BLACK (abyss pixels are black),
GEGL_ABYSS_CLAMP (coordinates are clamped to the abyss rectangle),
GEGL_ABYSS_LOOP (buffer contents are tiled if outside of the abyss rectangle)
this argument also takes a GEGL_BUFFER_FILTER value or'ed into it, allowing
to specify trade-off of performance/quality, valid values are:
GEGL_BUFFER_FILTER_NEAREST, GEGL_BUFFER_FILTER_BILINEAR,
GEGL_BUFFER_FILTER_BOX and GEGL_BUFFER_FILTER_AUTO.
<!-- impl Buffer::fn iterator_new -->
Create a new buffer iterator, this buffer will be iterated through
in linear chunks, some chunks might be full tiles the coordinates, see
the documentation of gegl_buffer_iterator_next for how to use it and
destroy it.
## `roi`
the rectangle to iterate over
## `level`
the level at which we are iterating, the roi will indicate the
extent at 1:1, x,y,width and height are/(2^level)
## `format`
the format we want to process this buffers data in, pass 0 to use the buffers format.
## `access_mode`
whether we need reading or writing to this buffer one of GEGL_BUFFER_READ, GEGL_BUFFER_WRITE and GEGL_BUFFER_READWRITE.
## `abyss_policy`
how request outside the buffer extent are handled.

# Returns

a new buffer iterator that can be used to iterate through the
buffers pixels.
<!-- impl Buffer::fn linear_close -->
This function makes sure GeglBuffer and underlying code is aware of changes
being made to the linear buffer. If the request was not a compatible one
it is written back to the buffer. Multiple concurrent users can be handed
the same buffer (both raw access and converted).
## `linear`
a previously returned buffer.
<!-- impl Buffer::fn linear_open -->
Raw direct random access to the full data of a buffer in linear memory.
## `extent`
region to open, pass NULL for entire buffer.
## `rowstride`
return location for rowstride.
## `format`
desired format or NULL to use buffers format.

# Returns

a pointer to a linear memory region describing the buffer, if the
request is compatible with the underlying data storage direct access
to the underlying data is provided. Otherwise, it returns a copy of the data.
<!-- impl Buffer::fn remove_handler -->
Remove the provided tile handler in the existing chain of tile handler of a GeglBuffer.
## `handler`
a [`TileHandler`][crate::TileHandler]
<!-- impl Buffer::fn sample -->
Query interpolate pixel values at a given coordinate using a specified form
of interpolation.

If you intend to take multiple samples, consider using
`gegl_buffer_sampler_new()` to create a sampler object instead, which is more
efficient.
## `x`
x coordinate to sample in buffer coordinates
## `y`
y coordinate to sample in buffer coordinates
## `scale`
a matrix that indicates scaling factors, see
gegl_sampler_compute_scale the same.
## `dest`
buffer capable of storing one pixel in `format`.
## `format`
the format to store the sampled color in.
## `sampler_type`
the sampler type to use,
to be ported from working code. Valid values: GEGL_SAMPLER_NEAREST,
GEGL_SAMPLER_LINEAR, GEGL_SAMPLER_CUBIC, GEGL_SAMPLER_NOHALO and
GEGL_SAMPLER_LOHALO
## `repeat_mode`
how requests outside the buffer extent are handled.
Valid values: GEGL_ABYSS_NONE (abyss pixels are zeroed), GEGL_ABYSS_WHITE
(abyss pixels are white), GEGL_ABYSS_BLACK (abyss pixels are black),
GEGL_ABYSS_CLAMP (coordinates are clamped to the abyss rectangle),
GEGL_ABYSS_LOOP (buffer contents are tiled if outside of the abyss rectangle).
<!-- impl Buffer::fn sample_at_level -->
Query interpolate pixel values at a given coordinate using a specified form
of interpolation.

If you intend to take multiple samples, consider using
`gegl_buffer_sampler_new_at_level()` to create a sampler object instead, which
is more efficient.
## `x`
x coordinate to sample in buffer coordinates
## `y`
y coordinate to sample in buffer coordinates
## `scale`
a matrix that indicates scaling factors, see
gegl_sampler_compute_scale the same.
## `dest`
buffer capable of storing one pixel in `format`.
## `format`
the format to store the sampled color in.
## `level`
mipmap level to sample from (`x` and `y` are level 0 coordinates)
## `sampler_type`
the sampler type to use,
to be ported from working code. Valid values: GEGL_SAMPLER_NEAREST,
GEGL_SAMPLER_LINEAR, GEGL_SAMPLER_CUBIC, GEGL_SAMPLER_NOHALO and
GEGL_SAMPLER_LOHALO
## `repeat_mode`
how requests outside the buffer extent are handled.
Valid values: GEGL_ABYSS_NONE (abyss pixels are zeroed), GEGL_ABYSS_WHITE
(abyss pixels are white), GEGL_ABYSS_BLACK (abyss pixels are black),
GEGL_ABYSS_CLAMP (coordinates are clamped to the abyss rectangle),
GEGL_ABYSS_LOOP (buffer contents are tiled if outside of the abyss rectangle).
<!-- impl Buffer::fn sampler_new -->
Create a new sampler, when you are done with the sampler, g_object_unref
it.

Samplers only hold weak references to buffers, so if its buffer is freed
the sampler will become invalid.
## `format`
format we want data back in
## `sampler_type`
the sampler type to use,
to be ported from working code. Valid values: GEGL_SAMPLER_NEAREST,
GEGL_SAMPLER_LINEAR, GEGL_SAMPLER_CUBIC, GEGL_SAMPLER_NOHALO and
GEGL_SAMPLER_LOHALO
<!-- impl Buffer::fn sampler_new_at_level -->
Create a new sampler, when you are done with the sampler, g_object_unref
it.

Samplers only hold weak references to buffers, so if its buffer is freed
the sampler will become invalid.
## `format`
format we want data back in
## `sampler_type`
the sampler type to use,
## `level`
the mipmap level to create a sampler for
to be ported from working code. Valid values: GEGL_SAMPLER_NEAREST,
GEGL_SAMPLER_LINEAR, GEGL_SAMPLER_CUBIC, GEGL_SAMPLER_NOHALO and
GEGL_SAMPLER_LOHALO
<!-- impl Buffer::fn set -->
Store a linear raster buffer into the GeglBuffer.
## `rect`
the coordinates we want to change the data of and the width/height of
the linear buffer being set.
the data when setting.
## `mipmap_level`
the scale level being set, 0 = 1:1 = default = base mipmap level,
1 = 1:2, 2=1:4, 3=1:8 ..
## `format`
the babl_format the linear buffer `src`.
## `src`
linear buffer of image data to be stored in `self`.
## `rowstride`
rowstride in bytes, or GEGL_AUTO_ROWSTRIDE to compute the
rowstride based on the width and bytes per pixel for the specified format.
<!-- impl Buffer::fn set_color_from_pixel -->
Sets the region covered by rect to the the provided pixel.
## `rect`
a rectangular region to fill with a color.
## `pixel`
pointer to the data of a single pixel
## `pixel_format`
the babl format of the pixel, if missing - the soft format of dst.
<!-- impl Buffer::fn signal_connect -->
This function should be used instead of g_signal_connect when connecting to
the GeglBuffer::changed signal handler, GeglBuffer contains additional
machinery to avoid the overhead of changes when no signal handler have been
connected, if regular g_signal_connect is used; then no signals will be
emitted.
## `detailed_signal`
only "changed" expected for now
## `c_handler`
c function callback

# Returns

an handle like g_signal_connect.
<!-- trait ColorExt::fn pixel -->
Store the color in a pixel in the given format.
## `format`
a babl pixel format
## `pixel`
pointer to a pixel
<!-- trait ColorExt::fn set_pixel -->
Set a GeglColor from a pointer to a pixel and it's babl format.
## `format`
a babl pixel format
## `pixel`
pointer to a pixel
<!-- trait MetadataExt::fn iter_get_value -->
Retrieve image file metadata from the application. Intended for use by the
image file writer. If the operation fails it returns [`false`] and `value` is
not updated.
## `iter`
`GeglMetadataIter` referencing the value to get
## `value`
Value to set in the interface

# Returns

[`true`] if successful.
<!-- trait MetadataExt::fn iter_init -->
Initialise an iterator to find all supported metadata keys.
## `iter`
`GeglMetadataIter` to be initialised
<!-- trait MetadataExt::fn iter_lookup -->
Look up the specified key and initialise an iterator to reference the
associated metadata. The iterator is used in conjunction with
`gegl_metadata_set_value()` and `gegl_metadata_get_value()`. Note that this
iterator is not valid for `gegl_metadata_iter_next()`.
## `iter`
`GeglMetadataIter` to be initialised
## `key`
Name of the value look up

# Returns

[`true`] if key is found.
<!-- trait MetadataExt::fn iter_next -->
Move the iterator to the next metadata item
## `iter`
`GeglMetadataIter` to be updated

# Returns

key name if found, else [`None`]
<!-- trait MetadataExt::fn iter_set_value -->
Set application data retrieved from image file's metadata. Intended for use
by the image file reader. If the operation fails it returns [`false`] and
`value` is ignored.
## `iter`
`GeglMetadataIter` referencing the value to set
## `value`
Value to set in the interface

# Returns

[`true`] if successful.
<!-- trait MetadataExt::fn register_map -->
Set the name of the file module and pass an array of mappings from
file-format specific metadata names to those used by Gegl. A GValue
transformation function may be supplied, e.g. to parse or format timestamps.
## `file_module`
String identifying the file module, e.g, `"gegl:png-save"`
## `flags`
Flags specifying capabilities of underlying file format
## `map`
Array of mappings from file module metadata
 names to Gegl well-known names.
<!-- trait MetadataStoreExt::fn value -->
Retrieve the metadata value. `value` must be initialised with a compatible
type. If the value is unset or has not been previously declared `value` is
unchanged and an error message is logged.
## `name`
Metadata name
## `value`
An initialised [`glib::Value`][crate::glib::Value].
<!-- trait MetadataStoreExt::fn register -->
## `local_name`
Metadata name known to file module
## `name`
Metadata name
## `transform`
A `GValueTransform` function or [`None`]
<!-- trait MetadataStoreExt::fn connect_generate_value -->
If a signal handler is connected to `::generate-value` a signal is emitted
when the file module accesses a value using `gegl_metadata_get_value()`.
The signal handler must generate a value of the type specified in the pspec
argument. The signal handler's return value indicates the success of the
operation.

If no handler is connected the mapped metadata value is accessed normally,
## `pspec`
A [`glib::ParamSpec`][crate::glib::ParamSpec] declaring the metadata value
## `value`
An initialised [`glib::Value`][crate::glib::Value].

# Returns

[`true`] if a value is generated successfully.
<!-- trait MetadataStoreExt::fn connect_parse_value -->
If a signal handler is connected to `::parse-value` a signal is emitted when
the file module accesses a value using `gegl_metadata_set_value()`. The
signal handler should parse the value supplied in the [`glib::Value`][crate::glib::Value] and may set
any number of metadata values using [`set_value()`][Self::set_value()].

If no handler is connected the mapped metadata value is set normally,
## `pspec`
A [`glib::ParamSpec`][crate::glib::ParamSpec] declaring the metadata value
## `value`
A [`glib::Value`][crate::glib::Value] containing the value to parse.

# Returns

[`true`] if parsing is successful.
<!-- trait MetadataStoreImpl::fn has_value -->
Test whether the [`MetadataStore`][crate::MetadataStore] contains a value for the specified name.
## `name`
Metadata name

# Returns

[`true`] if metadata is declared and contains a valid value.
<!-- trait MetadataStoreImpl::fn set_value -->
Set the specified metadata value. If `value` is [`None`] the default value from
the associated [`glib::ParamSpec`][crate::glib::ParamSpec] is used. This operation will fail if the value
has not been previously declared. A `changed::name` signal is emitted when
the value is set. If the value is shadowed by a property a `notify::name`
signal is also emitted.
## `name`
Metadata name
## `value`
(nullable): A valid [`glib::Value`][crate::glib::Value] or [`None`]
<!-- impl Path::fn add_flattener -->
Add a new flattener, the flattener should produce a type of path that
GeglPath already understands, if the flattener is unable to flatten
the incoming path (doesn't understand the instructions), the original
path should be returned.
## `func`
a `GeglFlattenerFunc`
<!-- impl Path::fn append -->
Use as follows: gegl_path_append (path, 'M', 0.0, 0.0);
and gegl_path_append (path, 'C', 10.0, 10.0, 50.0, 10.0, 60.0, 0.0) the
number of arguments are determined from the instruction provided.
<!-- impl Path::fn foreach -->
Execute a provided function for every node in the path (useful for
drawing and otherwise traversing a path.)
## `each_item`
a function to call for each node in the path.
<!-- impl Path::fn foreach_flat -->
Execute a provided function for the segments of a poly line approximating
the path.
## `each_item`
a function to call for each node in the path.
<!-- impl Path::fn flat_path -->
Return a polyline version of `self`
<!-- impl Path::fn node -->
Retrieve the node of the path at position `pos`.

Returns TRUE if the node was successfully retrieved.
## `index`
the node number to retrieve

# Returns


## `node`
a pointer to a `GeglPathItem` record to be written.
<!-- impl Path::fn path -->
Return the internal untouched `GeglPathList`
<!-- impl Path::fn insert_node -->
Insert the new node `node` at position `pos` in `self`.
if `pos` = -1, the node is added in the last position.
## `pos`
the position we want the new node to have.
## `node`
pointer to a structure describing the GeglPathItem we want to store
<!-- impl Path::fn replace_node -->
Replaces the exiting node at position `pos` in `self`.
## `pos`
the position we want the new node to have.
## `node`
pointer to a structure describing the GeglPathItem we want to store.
<!-- trait TileBackendExt::fn command -->
The default tile-backend command handler. Tile backends should forward
commands they don't handle themselves to this function.
## `command`
the tile command
## `x`
x coordinate
## `y`
y coordinate
## `z`
tile zoom level

# Returns

Command result.
<!-- trait TileHandlerExt::fn create_tile -->
Create a new tile associated with this tile handler.
## `x`
The tile space x coordinate for the tile
## `y`
The tile space y coordinate for the tile
## `z`
The tile space z coordinate for the tile

# Returns

the new tile
<!-- trait TileHandlerExt::fn dup_tile -->
Create a duplicate of `tile`, associated with this tile handler.
## `tile`
the `GeglTile` to copy
## `x`
The tile space x coordinate for the tile
## `y`
The tile space y coordinate for the tile
## `z`
The tile space z coordinate for the tile

# Returns

the new tile
<!-- trait TileHandlerExt::fn source_tile -->
Fetches the tile at the given coordinates from `self` source. If the tile
doesn't exist, or if `self` doesn't have a source, creates a new tile
associated with this tile handler.

If `preserve_data` is FALSE, the tile contents are unspecified.
## `x`
The tile space x coordinate for the tile
## `y`
The tile space y coordinate for the tile
## `z`
The tile space z coordinate for the tile
## `preserve_data`
whether existing tile data should be preserved

# Returns

the tile
<!-- trait TileHandlerExt::fn tile -->
Fetches the tile at the given coordinates from `self`. If the tile
doesn't exist, creates a new tile associated with this tile handler.

If `preserve_data` is FALSE, the tile contents are unspecified.
## `x`
The tile space x coordinate for the tile
## `y`
The tile space y coordinate for the tile
## `z`
The tile space z coordinate for the tile
## `preserve_data`
whether existing tile data should be preserved

# Returns

the tile
