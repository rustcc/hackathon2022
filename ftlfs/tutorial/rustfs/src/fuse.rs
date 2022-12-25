#![allow(dead_code, non_camel_case_types)]

use libc::{self, c_char, c_int, c_uint, c_ulong, c_void, off_t, size_t};

#[repr(C)]
pub struct fuse_args {
    /** Argument count */
    pub argc: c_int,
    /** Argument vector.  NULL terminated */
    pub argv: *mut *mut c_char,
    /** Is 'argv' allocated? */
    pub allocated: c_int,
}

#[repr(C)]
pub struct fuse_opt {
    pub templ: *const c_char,
    /**
     * Offset of variable within 'data' parameter of fuse_opt_parse()
     * or -1
     */
    pub offset: c_ulong,
    /**
     * Value to set the variable to, or to be passed as 'key' to the
     * processing function.Ignored if template has a format
     */
    pub value: c_int,
}

#[repr(C)]
pub struct fuse_pollhandle;

#[repr(C)]
pub struct fuse_buf;

#[repr(C)]
pub struct fuse_bufvec;

#[repr(C)]
pub struct fuse_file_info {
    /** Open flags.Available in open() and release() */
    // int flags;
    pub flags: c_int,

    /** Old file handle, don't use */
    // unsigned long fh_old;
    pub fh_old: c_ulong,

    /** In case of a write operation indicates if this was caused by a
    writepage */
    // int writepage;
    pub writepage: c_int,

    // =================================================================
    // /** Can be filled in by open, to use direct I/O on this file.
    //     Introduced in version 2.4 */
    // // unsigned int direct_io : 1;
    // unsigned int direct_io : 1;

    // /** Can be filled in by open, to indicate, that cached file data
    //     need not be invalidated.  Introduced in version 2.4 */
    // unsigned int keep_cache : 1;

    // /** Indicates a flush operation.  Set in flush operation, also
    //     maybe set in highlevel lock operation and lowlevel release
    //     operation.	Introduced in version 2.6 */
    // unsigned int flush : 1;

    // /** Can be filled in by open, to indicate that the file is not
    //     seekable.  Introduced in version 2.8 */
    // unsigned int nonseekable : 1;

    // /* Indicates that flock locks for this file should be
    //    released.  If set, lock_owner shall contain a valid value.
    //    May only be set in ->release().  Introduced in version
    //    2.9 */
    // unsigned int flock_release : 1;

    // /** Padding.  Do not use*/
    // unsigned int padding : 27;
    // =================================================================
    pub bitfield: u32,

    /** File handle.  May be filled in by filesystem in open().
    Available in all other file operations */
    // uint64_t fh;
    pub fh: u64,

    /** Lock owner id.  Available in locking operations and flush */
    // uint64_t lock_owner;
    pub lock_owner: u64,
}

#[repr(C)]
pub struct fuse_conn_info {
    /**
     * Major version of the protocol (read-only)
     */
    // unsigned proto_major;
    pub proto_major: c_uint,

    /**
     * Minor version of the protocol (read-only)
     */
    // unsigned proto_minor;
    pub proto_minor: c_uint,

    /**
     * Is asynchronous read supported (read-write)
     */
    // unsigned async_read;
    pub async_read: c_uint,

    /**
     * Maximum size of the write buffer
     */
    // unsigned max_write;
    pub max_write: c_uint,

    /**
     * Maximum readahead
     */
    // unsigned max_readahead;
    pub max_readahead: c_uint,

    /**
     * Capability flags, that the kernel supports
     */
    // unsigned capable;
    pub capable: c_uint,

    /**
     * Capability flags, that the filesystem wants to enable
     */
    // unsigned want;
    pub want: c_uint,

    /**
     * Maximum number of backgrounded requests
     */
    // unsigned max_background;
    pub max_background: c_uint,

    /**
     * Kernel congestion threshold parameter
     */
    // unsigned congestion_threshold;
    pub congestion_threshold: c_uint,

    /**
     * For future use.
     */
    // unsigned reserved[23];
    pub reserved: [c_uint; 23],
}

#[repr(C)]
pub struct fuse_operations {
    /** Get file attributes.
     *
     * Similar to stat().  The 'st_dev' and 'st_blksize' fields are
     * ignored. The 'st_ino' field is ignored except if the 'use_ino'
     * mount option is given.
     */
    // int (*getattr) (const char *, struct stat *);
    pub getattr: Option<unsafe extern "C" fn(path: *const c_char, stat: *mut libc::stat) -> c_int>,

    /** Read the target of a symbolic link
     *
     * The buffer should be filled with a null terminated string.  The
     * buffer size argument includes the space for the terminating
     * null character.If the linkname is too long to fit in the
     * buffer, it should be truncated.The return value should be 0
     * for success.
     */
    // int (*readlink) (const char *, char *, size_t);
    pub readlink: Option<extern "C" fn(*const c_char, *mut c_char, libc::size_t) -> c_int>,

    /* Deprecated, use readdir() instead */
    // int (*getdir) (const char *, fuse_dirh_t, fuse_dirfil_t);
    pub getdir: Option<extern "C" fn()>,

    /** Create a file node
     *
     * This is called for creation of all non-directory, non-symlink
     * nodes.  If the filesystem defines a create() method, then for
     * regular files that will be called instead.
     */
    // int (*mknod) (const char *, mode_t, dev_t);
    pub mknod: Option<extern "C" fn(path: *const c_char, libc::mode_t, libc::dev_t) -> c_int>,

    /** Create a directory
     *
     * Note that the mode argument may not have the type specification
     * bits set, i.e. S_ISDIR(mode) can be false.  To obtain the
     * correct directory type bits use  mode|S_IFDIR
     * */
    // int (*mkdir) (const char *, mode_t);
    pub mkdir: Option<extern "C" fn(path: *const c_char, libc::mode_t) -> c_int>,

    /** Remove a file */
    // int (*unlink) (const char *);
    pub unlink: Option<extern "C" fn(path: *const c_char) -> c_int>,

    /** Remove a directory */
    // int (*rmdir) (const char *);
    pub rmdir: Option<extern "C" fn(path: *const c_char) -> c_int>,

    /** Create a symbolic link */
    // int (*symlink) (const char *, const char *);
    pub symlink: Option<extern "C" fn(*const c_char, *const c_char) -> c_int>,

    /** Rename a file */
    // int (*rename) (const char *, const char *);
    pub rename: Option<extern "C" fn(old_name: *const c_char, new_name: *const c_char) -> c_int>,

    /** Create a hard link to a file */
    // int (*link) (const char *, const char *);
    pub link: Option<extern "C" fn(old_path: *const c_char, new_path: *const c_char) -> c_int>,

    /** Change the permission bits of a file */
    // int (*chmod) (const char *, mode_t);
    pub chmod: Option<extern "C" fn(path: *const c_char, libc::mode_t) -> c_int>,

    /** Change the owner and group of a file */
    // int (*chown) (const char *, uid_t, gid_t);
    pub chown: Option<extern "C" fn(path: *const c_char, libc::uid_t, libc::gid_t) -> c_int>,

    /** Change the size of a file */
    // int (*truncate) (const char *, off_t);
    pub truncate: Option<extern "C" fn(path: *const c_char, libc::off_t) -> c_int>,

    /** Change the access and/or modification times of a file
     *
     * Deprecated, use utimens() instead.
     */
    // int (*utime) (const char *, struct utimbuf *);
    pub utime: Option<extern "C" fn(path: *const c_char, *mut libc::utimbuf) -> c_int>,

    /** File open operation
     *
     * No creation (O_CREAT, O_EXCL) and by default also no
     * truncation (O_TRUNC) flags will be passed to open(). If an
     * application specifies O_TRUNC, fuse first calls truncate()
     * and then open(). Only if 'atomic_o_trunc' has been
     * specified and kernel version is 2.6.24 or later, O_TRUNC is
     * passed on to open.
     *
     * Unless the 'default_permissions' mount option is given,
     * open should check if the operation is permitted for the
     * given flags. Optionally open may also return an arbitrary
     * filehandle in the fuse_file_info structure, which will be
     * passed to all file operations.
     *
     * Changed in version 2.2
     */
    // int (*open) (const char *, struct fuse_file_info *);
    pub open: Option<extern "C" fn(path: *const c_char, *mut fuse_file_info) -> c_int>,

    /** Read data from an open file
     *
     * Read should return exactly the number of bytes requested except
     * on EOF or error, otherwise the rest of the data will be
     * substituted with zeroes.An exception to this is when the
     * 'direct_io' mount option is specified, in which case the return
     * value of the read system call will reflect the return value of
     * this operation.
     *
     * Changed in version 2.2
     */
    // int (*read) (const char *, char *, size_t, off_t, struct fuse_file_info *);
    pub read: Option<
        extern "C" fn(
            path: *const c_char,
            dst: *mut c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> c_int,
    >,

    /** Write data to an open file
     *
     * Write should return exactly the number of bytes requested
     * except on error. An exception to this is when the 'direct_io'
     * mount option is specified (see read operation).
     *
     * Changed in version 2.2
     */
    // int (*write) (const char *, const char *, size_t, off_t, struct fuse_file_info *);
    pub write: Option<
        extern "C" fn(
            path: *const c_char,
            src: *const c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> c_int,
    >,

    /** Get file system statistics
     *
     * The 'f_frsize', 'f_favail', 'f_fsid' and 'f_flag' fields are ignored
     *
     * Replaced 'struct statfs' parameter with 'struct statvfs' in
     * version 2.5
     */
    // int (*statfs) (const char *, struct statvfs *);
    pub statfs: Option<extern "C" fn(path: *const c_char, libc::statvfs) -> c_int>,

    /** Possibly flush cached data
     *
     * BIG NOTE: This is not equivalent to fsync().  It's not a
     * request to sync dirty data.
     *
     * Flush is called on each close() of a file descriptor.  So if a
     * filesystem wants to return write errors in close() and the file
     * has cached dirty data, this is a good place to write back data
     * and return any errors.  Since many applications ignore close()
     * errors this is not always useful.
     *
     * NOTE: The flush() method may be called more than once for each
     * open().This happens if more than one file descriptor refers
     * to an opened file due to dup(), dup2() or fork() calls.It is
     * not possible to determine if a flush is final, so each flush
     * should be treated equally.  Multiple write-flush sequences are
     * relatively rare, so this shouldn't be a problem.
     *
     * Filesystems shouldn't assume that flush will always be called
     * after some writes, or that if will be called at all.
     *
     * Changed in version 2.2
     */
    // int (*flush) (const char *, struct fuse_file_info *);
    pub flush: Option<extern "C" fn(path: *const c_char, *mut fuse_file_info) -> c_int>,

    /** Release an open file
     *
     * Release is called when there are no more references to an open
     * file: all file descriptors are closed and all memory mappings
     * are unmapped.
     *
     * For every open() call there will be exactly one release() call
     * with the same flags and file descriptor. It is possible to
     * have a file opened more than once, in which case only the last
     * release will mean, that no more reads/writes will happen on the
     * file.  The return value of release is ignored.
     *
     * Changed in version 2.2
     */
    // int (*release) (const char *, struct fuse_file_info *);
    pub release: Option<extern "C" fn(path: *const c_char, *mut fuse_file_info) -> c_int>,

    /** Synchronize file contents
     *
     * If the datasync parameter is non-zero, then only the user data
     * should be flushed, not the meta data.
     *
     * Changed in version 2.2
     */
    // int (*fsync) (const char *, int, struct fuse_file_info *);
    pub fsync: Option<extern "C" fn(path: *const c_char, c_int, *mut fuse_file_info) -> c_int>,

    /** Set extended attributes */
    // int (*setxattr) (const char *, const char *, const char *, size_t, int);
    pub setxattr:
        Option<extern "C" fn(*const c_char, *const c_char, *const c_char, size_t, c_int) -> c_int>,

    /** Get extended attributes */
    // int (*getxattr) (const char *, const char *, char *, size_t);
    pub getxattr: Option<extern "C" fn(*const c_char, *const c_char, *mut c_char, size_t) -> c_int>,

    /** List extended attributes */
    // int (*listxattr) (const char *, char *, size_t);
    pub listxattr: Option<extern "C" fn(*const c_char, *mut c_char, size_t) -> c_int>,

    // /** Remove extended attributes */
    // int (*removexattr) (const char *, const char *);
    pub removexattr: Option<extern "C" fn(*const c_char, *const c_char) -> c_int>,

    /** Open directory
     *
     * Unless the 'default_permissions' mount option is given,
     * this method should check if opendir is permitted for this
     * directory. Optionally opendir may also return an arbitrary
     * filehandle in the fuse_file_info structure, which will be
     * passed to readdir, releasedir and fsyncdir.
     *
     * Introduced in version 2.3
     */
    // int (*opendir) (const char *, struct fuse_file_info *);
    pub opendir: Option<extern "C" fn(*const c_char, *mut fuse_file_info) -> c_int>,

    /** Read directory
     *
     * This supersedes the old getdir() interface.  New applications
     * should use this.
     *
     * The filesystem may choose between two modes of operation:
     *
     * 1) The readdir implementation ignores the offset parameter, and
     * passes zero to the filler function's offset.  The filler
     * function will not return '1' (unless an error happens), so the
     * whole directory is read in a single readdir operation.  This
     * works just like the old getdir() method.
     *
     * 2) The readdir implementation keeps track of the offsets of the
     * directory entries.  It uses the offset parameter and always
     * passes non-zero offset to the filler function.  When the buffer
     * is full (or an error happens) the filler function will return
     * '1'.
     *
     * Introduced in version 2.3
     */
    // int (*readdir) (const char *, void *, fuse_fill_dir_t, off_t, struct fuse_file_info *);
    pub readdir: Option<
        extern "C" fn(
            *const c_char,
            *mut c_void,
            fuse_fill_dir_t,
            off_t,
            *mut fuse_file_info,
        ) -> c_int,
    >,

    /** Release directory
     *
     * Introduced in version 2.3
     */
    // int (*releasedir) (const char *, struct fuse_file_info *);
    pub releasedir: Option<extern "C" fn(*const c_char, *mut fuse_file_info) -> c_int>,

    /** Synchronize directory contents
     *
     * If the datasync parameter is non-zero, then only the user data
     * should be flushed, not the meta data
     *
     * Introduced in version 2.3
     */
    // int (*fsyncdir) (const char *, int, struct fuse_file_info *);
    pub fsyncdir: Option<extern "C" fn(*const c_char, c_int, *mut fuse_file_info) -> c_int>,

    /**
     * Initialize filesystem
     *
     * The return value will passed in the private_data field of
     * fuse_context to all file operations and as a parameter to the
     * destroy() method.
     *
     * Introduced in version 2.3
     * Changed in version 2.6
     */
    // void *(*init) (struct fuse_conn_info *conn);
    pub init: Option<extern "C" fn(conn: *mut fuse_conn_info) -> c_int>,

    /**
     * Clean up filesystem
     *
     * Called on filesystem exit.
     *
     * Introduced in version 2.3
     */
    // void (*destroy) (void *);
    pub destroy: Option<extern "C" fn(*mut c_void)>,

    /**
     * Check file access permissions
     *
     * This will be called for the access() system call.  If the
     * 'default_permissions' mount option is given, this method is not
     * called.
     *
     * This method is not called under Linux kernel versions 2.4.x
     *
     * Introduced in version 2.5
     */
    // int (*access) (const char *, int);
    pub access: Option<extern "C" fn(*const c_char, c_int) -> c_int>,

    /**
     * Create and open a file
     *
     * If the file does not exist, first create it with the specified
     * mode, and then open it.
     *
     * If this method is not implemented or under Linux kernel
     * versions earlier than 2.6.15, the mknod() and open() methods
     * will be called instead.
     *
     * Introduced in version 2.5
     */
    // int (*create) (const char *, mode_t, struct fuse_file_info *);
    pub create: Option<extern "C" fn(*const c_char, libc::mode_t, *mut fuse_file_info) -> c_int>,

    /**
     * Change the size of an open file
     *
     * This method is called instead of the truncate() method if the
     * truncation was invoked from an ftruncate() system call.
     *
     * If this method is not implemented or under Linux kernel
     * versions earlier than 2.6.15, the truncate() method will be
     * called instead.
     *
     * Introduced in version 2.5
     */
    // int (*ftruncate) (const char *, off_t, struct fuse_file_info *);
    pub ftruncate: Option<extern "C" fn(*const c_char, off_t, *mut fuse_file_info) -> c_int>,

    // /**
    //  * Get attributes from an open file
    //  *
    //  * This method is called instead of the getattr() method if the
    //  * file information is available.
    //  *
    //  * Currently this is only called after the create() method if that
    //  * is implemented (see above).  Later it may be called for
    //  * invocations of fstat() too.
    //  *
    //  * Introduced in version 2.5
    //  */
    // int (*fgetattr) (const char *, struct stat *, struct fuse_file_info *);
    pub fgetattr:
        Option<extern "C" fn(*const c_char, *mut libc::stat, *mut fuse_file_info) -> c_int>,

    /**
     * Perform POSIX file locking operation
     *
     * The cmd argument will be either F_GETLK, F_SETLK or F_SETLKW.
     *
     * For the meaning of fields in 'struct flock' see the man page
     * for fcntl(2).  The l_whence field will always be set to
     * SEEK_SET.
     *
     * For checking lock ownership, the 'fuse_file_info->owner'
     * argument must be used.
     *
     * For F_GETLK operation, the library will first check currently
     * held locks, and if a conflicting lock is found it will return
     * information without calling this method. This ensures, that
     * for local locks the l_pid field is correctly filled in.The
     * results may not be accurate in case of race conditions and in
     * the presence of hard links, but it's unlikely that an
     * application would rely on accurate GETLK results in these
     * cases.  If a conflicting lock is not found, this method will be
     * called, and the filesystem may fill out l_pid by a meaningful
     * value, or it may leave this field zero.
     *
     * For F_SETLK and F_SETLKW the l_pid field will be set to the pid
     * of the process performing the locking operation.
     *
     * Note: if this method is not implemented, the kernel will still
     * allow file locking to work locally.  Hence it is only
     * interesting for network filesystems and similar.
     *
     * Introduced in version 2.6
     */
    // int (*lock) (const char *, struct fuse_file_info *, int cmd, struct flock *);
    pub lock: Option<
        extern "C" fn(*const c_char, *mut fuse_file_info, cmd: c_int, *mut libc::flock) -> c_int,
    >,

    /**
     * Change the access and modification times of a file with
     * nanosecond resolution
     *
     * This supersedes the old utime() interface.  New applications
     * should use this.
     *
     * See the utimensat(2) man page for details.
     *
     * Introduced in version 2.6
     */
    // int (*utimens) (const char *, const struct timespec tv[2]);
    pub utimens: Option<extern "C" fn(*const c_char, *const [libc::timespec; 2]) -> c_int>,

    /**
     * Map block index within file to block index within device
     *
     * Note: This makes sense only for block device backed filesystems
     * mounted with the 'blkdev' option
     *
     * Introduced in version 2.6
     */
    // int (*bmap) (const char *, size_t blocksize, uint64_t *idx);
    pub bmap: Option<extern "C" fn(*const c_char, blocksize: size_t, idx: *mut u64) -> c_int>,

    // ==================================================================
    // /**
    //  * Flag indicating that the filesystem can accept a NULL path
    //  * as the first argument for the following operations:
    //  *
    //  * read, write, flush, release, fsync, readdir, releasedir,
    //  * fsyncdir, ftruncate, fgetattr, lock, ioctl and poll
    //  *
    //  * If this flag is set these operations continue to work on
    //  * unlinked files even if "-ohard_remove" option was specified.
    //  */
    // unsigned int flag_nullpath_ok:1;

    // /**
    //  * Flag indicating that the path need not be calculated for
    //  * the following operations:
    //  *
    //  * read, write, flush, release, fsync, readdir, releasedir,
    //  * fsyncdir, ftruncate, fgetattr, lock, ioctl and poll
    //  *
    //  * Closely related to flag_nullpath_ok, but if this flag is
    //  * set then the path will not be calculaged even if the file
    //  * wasn't unlinked.  However the path can still be non-NULL if
    //  * it needs to be calculated for some other reason.
    //  */
    // unsigned int flag_nopath:1;

    // /**
    //  * Flag indicating that the filesystem accepts special
    //  * UTIME_NOW and UTIME_OMIT values in its utimens operation.
    //  */
    // unsigned int flag_utime_omit_ok:1;

    // /**
    //  * Reserved flags, don't set
    //  */
    // unsigned int flag_reserved:29;

    // ==================================================================
    pub bitfield: u32,

    // /**
    //  * Ioctl
    //  *
    //  * flags will have FUSE_IOCTL_COMPAT set for 32bit ioctls in
    //  * 64bit environment.  The size and direction of data is
    //  * determined by _IOC_*() decoding of cmd.  For _IOC_NONE,
    //  * data will be NULL, for _IOC_WRITE data is out area, for
    //  * _IOC_READ in area and if both are set in/out area.  In all
    //  * non-NULL cases, the area is of _IOC_SIZE(cmd) bytes.
    //  *
    //  * If flags has FUSE_IOCTL_DIR then the fuse_file_info refers to a
    //  * directory file handle.
    //  *
    //  * Introduced in version 2.8
    //  */
    // int (*ioctl) (const char *, int cmd, void *arg,
    // 	      struct fuse_file_info *, unsigned int flags, void *data);
    pub ioctl: Option<
        extern "C" fn(
            *const c_char,
            cmd: c_int,
            arg: *mut c_void,
            *mut fuse_file_info,
            flags: c_uint,
            data: *mut c_void,
        ) -> i32,
    >,
    /**
     * Poll for IO readiness events
     *
     * Note: If ph is non-NULL, the client should notify
     * when IO readiness events occur by calling
     * fuse_notify_poll() with the specified ph.
     *
     * Regardless of the number of times poll with a non-NULL ph
     * is received, single notification is enough to clear all.
     * Notifying more times incurs overhead but doesn't harm
     * correctness.
     *
     * The callee is responsible for destroying ph with
     * fuse_pollhandle_destroy() when no longer in use.
     *
     * Introduced in version 2.8
     */
    // int (*poll) (const char *, struct fuse_file_info *,
    // 	     struct fuse_pollhandle *ph, unsigned *reventsp);
    pub poll: Option<
        extern "C" fn(
            *const c_char,
            *mut fuse_file_info,
            ph: *mut fuse_pollhandle,
            reventsp: *mut c_uint,
        ) -> i32,
    >,

    /** Write contents of buffer to an open file
     *
     * Similar to the write() method, but data is supplied in a
     * generic buffer.  Use fuse_buf_copy() to transfer data to
     * the destination.
     *
     * Introduced in version 2.9
     */
    // int (*write_buf) (const char *, struct fuse_bufvec *buf, off_t off,
    // 		  struct fuse_file_info *);
    pub write_buf: Option<
        extern "C" fn(*const c_char, *mut fuse_bufvec, off: off_t, *mut fuse_file_info) -> i32,
    >,
    /** Store data from an open file in a buffer
     *
     * Similar to the read() method, but data is stored and
     * returned in a generic buffer.
     *
     * No actual copying of data has to take place, the source
     * file descriptor may simply be stored in the buffer for
     * later data transfer.
     *
     * The buffer must be allocated dynamically and stored at the
     * location pointed to by bufp.  If the buffer contains memory
     * regions, they too must be allocated using malloc().  The
     * allocated memory will be freed by the caller.
     *
     * Introduced in version 2.9
     */
    // int (*read_buf) (const char *, struct fuse_bufvec **bufp,
    // 		 size_t size, off_t off, struct fuse_file_info *);
    pub read_buf: Option<
        extern "C" fn(
            *const c_char,
            *mut fuse_bufvec,
            size: size_t,
            off: off_t,
            *mut fuse_file_info,
        ) -> i32,
    >,
    /**
     * Perform BSD file locking operation
     *
     * The op argument will be either LOCK_SH, LOCK_EX or LOCK_UN
     *
     * Nonblocking requests will be indicated by ORing LOCK_NB to
     * the above operations
     *
     * For more information see the flock(2) manual page.
     *
     * Additionally fi->owner will be set to a value unique to
     * this open file.  This same value will be supplied to
     * ->release() when the file is released.
     *
     * Note: if this method is not implemented, the kernel will still
     * allow file locking to work locally.  Hence it is only
     * interesting for network filesystems and similar.
     *
     * Introduced in version 2.9
     */
    // int (*flock) (const char *, struct fuse_file_info *, int op);
    pub flock: Option<extern "C" fn(*const c_char, *mut fuse_file_info, op: c_int) -> i32>,
    /**
     * Allocates space for an open file
     *
     * This function ensures that required space is allocated for specified
     * file.  If this function returns success then any subsequent write
     * request to specified range is guaranteed not to fail because of lack
     * of space on the file system media.
     *
     * Introduced in version 2.9.1
     */
    // int (*fallocate) (const char *, int, off_t, off_t,
    // 		  struct fuse_file_info *);
    pub fallocate:
        Option<extern "C" fn(*const c_char, c_int, off_t, off_t, *mut fuse_file_info) -> i32>,
}

impl fuse_operations {
    pub fn empty() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

pub type fuse_opt_proc_t = Option<
    extern "C" fn(
        data: *mut c_void,
        arg: *const c_char,
        key: c_int,
        outargs: *mut fuse_args,
    ) -> c_int,
>;
// typedef int (*fuse_fill_dir_t) (void *buf, const char *name, const struct stat *stbuf, off_t off);
pub type fuse_fill_dir_t = extern "C" fn(
    buf: *mut c_void,
    name: *const c_char,
    stbuf: *const libc::stat,
    off: off_t,
) -> c_int;

#[link(name = "ddriver", kind = "static")]
extern "C" {
    // int fuse_opt_parse(struct fuse_args *args, void *data, const struct fuse_opt opts[], fuse_opt_proc_t proc);
    pub fn fuse_opt_parse(
        args: *mut fuse_args,
        data: *mut c_void,
        fuse_opt: *const fuse_opt,
        proc: fuse_opt_proc_t,
    ) -> c_int;
    // int fuse_main_real(int argc, char *argv[], const struct fuse_operations *op, size_t op_size, void *user_data);
    #[allow(improper_ctypes)]
    pub fn fuse_main_real(
        argc: c_int,
        argv: *const *const c_char,
        op: *const fuse_operations,
        op_size: size_t,
        user_data: *mut c_void,
    ) -> c_int;
    // void fuse_opt_free_args(struct fuse_args *args);
    pub fn fuse_opt_free_args(args: *mut fuse_args);
}
