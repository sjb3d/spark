pub fn make_version(major: u32, minor: u32, patch: u32) Version {
    return Version{
        .major = @truncate(major),
        .minor = @truncate(minor),
        .patch = @truncate(patch),
    };
}

pub const Bool32 = enum(u32) {
    false = 0,
    true = 1,
    _,

    pub fn from_bool(b: bool) Bool32 {
        return if (b) .true else .false;
    }
    pub fn to_bool(self: Bool32) bool {
        return self != .false;
    }
};

pub const Version = packed struct(u32) {
    patch: u12 = 0,
    minor: u10 = 0,
    major: u7 = 0,
    variant: u3 = 0,

    pub fn to_int(self: Version) u32 {
        return @bitCast(self);
    }
    pub fn from_int(i: u32) Version {
        return @bitCast(i);
    }

    pub fn format(self: Version, writer: *std.io.Writer) std.io.Writer.Error!void {
        return writer.print("{}.{}.{}", .{ self.major, self.minor, self.patch });
    }
};

pub const MissingFunctionError = error{
    MissingFunction,
};

pub const Loader = struct {
    lib: std.DynLib,
    fp_get_instance_proc_addr: FpGetInstanceProcAddr,

    pub fn init() !Loader {
        var lib = try std.DynLib.open("vulkan-1.dll");
        const fp_get_instance_proc_addr = lib.lookup(FpGetInstanceProcAddr, "vkGetInstanceProcAddr") orelse return error.MissingFunction;

        return Loader{
            .lib = lib,
            .fp_get_instance_proc_addr = fp_get_instance_proc_addr,
        };
    }

    pub fn get_instance_proc_addr(self: Loader, p_name: [*:0]const u8) MissingFunctionError!FpVoidFunction {
        return self.fp_get_instance_proc_addr(.null_handle, p_name) orelse return error.MissingFunction;
    }
};

fn BitField(comptime Fields: type) type {
    const bit_count = 1 << @bitSizeOf(@typeInfo(Fields).@"enum".tag_type);
    const BitsInt = std.meta.Int(.unsigned, bit_count);

    return packed struct(BitsInt) {
        bits: BitsInt,

        const Self = @This();

        pub const none: Self = .{ .bits = 0 };

        pub fn init(init_values: std.enums.EnumFieldStruct(Fields, bool, false)) Self {
            var result: Self = .none;
            inline for (std.meta.fields(Fields)) |field| {
                if (@field(init_values, field.name)) {
                    result.set(@field(Fields, field.name));
                }
            }
            return result;
        }
        pub fn init_one(field: Fields) Self {
            return Self{ .bits = bit_from_field(field) };
        }

        fn bit_from_field(field: Fields) BitsInt {
            return @as(BitsInt, 1) << @intFromEnum(field);
        }

        pub fn set(self: *Self, field: Fields) void {
            self.bits |= bit_from_field(field);
        }
        pub fn unset(self: *Self, field: Fields) void {
            self.bits &= ~bit_from_field(field);
        }

        pub fn intersect_with(self: *Self, other: Self) void {
            self.bits &= other.bits;
        }
        pub fn union_with(self: *Self, other: Self) void {
            self.bits |= other.bits;
        }

        pub fn intersected_with(self: Self, other: Self) Self {
            return Self{ .bits = self.bits & other.bits };
        }
        pub fn unioned_with(self: Self, other: Self) Self {
            return Self{ .bits = self.bits | other.bits };
        }

        pub fn eql(self: Self, other: Self) bool {
            return self.bits == other.bits;
        }

        pub fn is_set(self: Self, field: Fields) bool {
            return (self.bits & bit_from_field(field)) != 0;
        }
        pub fn is_none(self: Self) bool {
            return self.bits == 0;
        }
        pub fn is_superset_of(self: Self, other: Self) bool {
            return self.intersected_with(other).eql(other);
        }
        pub fn is_subset_of(self: Self, other: Self) bool {
            return self.intersected_with(other).eql(self);
        }
    };
}

pub const EnumerateResult = enum {
    success,
    incomplete,
};

fn enumerate_generic_to_array(
    comptime ErrorSet: type,
    comptime Element: type,
    enumerator: anytype,
    allocator: Allocator,
) ErrorSet![]Element {
    while (true) {
        var len: u32 = undefined;
        const len_result = try enumerator.enumerate(&len, null);
        switch (len_result) {
            .success => {},
            .incomplete => return error.Unexpected,
        }
        var items = try allocator.alloc(Element, len);
        errdefer allocator.free(items);
        const items_result = try enumerator.enumerate(&len, items.ptr);
        switch (items_result) {
            .success => return items[0..len],
            .incomplete => allocator.free(items),
        }
    }
}

fn enumerate_generic_unchecked_to_array(
    comptime Element: type,
    enumerator: anytype,
    allocator: Allocator,
) Allocator.Error![]Element {
    var len: u32 = undefined;
    enumerator.enumerate(&len, null);
    var items = try allocator.alloc(Element, len);
    enumerator.enumerate(&len, items.ptr);
    return items[0..len];
}
