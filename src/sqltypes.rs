use winapi::*;

pub type SQLCHAR = c_uchar;
pub type SQLSCHAR = c_schar;
pub type SQLDATE = c_uchar;
pub type SQLDECIMAL = c_uchar;
pub type SQLDOUBLE = c_double;
pub type SQLFLOAT = c_double;
pub type SQLINTEGER = c_long;
pub type SQLUINTEGER = c_ulong;

pub type SQLLEN = INT64;
pub type SQLULEN = UINT64;
pub type SQLSETPOSIROW = UINT64;

pub type SQLROWCOUNT = SQLULEN;
pub type SQLROWSETSIZE = SQLULEN;
pub type SQLTRANSID = SQLULEN;
pub type SQLROWOFFSET = SQLLEN;

pub type SQLNUMERIC = c_uchar;
pub type SQLPOINTER = c_void;
pub type SQLREAL = c_float;
pub type SQLSMALLINT = c_short;
pub type SQLUSMALLINT = c_ushort;
pub type SQLTIME = c_uchar;
pub type SQLTIMESTAMP = c_uchar;
pub type SQLVARCHAR = c_uchar;


pub type SQLRETURN = SQLSMALLINT;
pub type SQLHANDLE = c_void;

pub type SQLHENV = SQLHANDLE;
pub type SQLHDBC = SQLHANDLE;
pub type SQLHSTMT = SQLHANDLE;
pub type SQLHDESC = SQLHANDLE;

pub type SQLHWND = HWND;