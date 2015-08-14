#![allow(bad_style, unused_variables)]

extern crate winapi;
mod sqltypes;
mod sql;
mod sqlext;

use sqltypes::*;
use sql::*;

#[no_mangle]
pub extern fn SQLAllocHandle(
	handleType: SQLSMALLINT,
	inputHandle: SQLHANDLE,
	outputHandlePtr: *mut SQLHANDLE) -> SQLRETURN {

	return SQL_SUCCESS;
}

#[no_mangle]
pub extern fn SQLBindCol(
	statementHandle: SQLHSTMT,
	columnNumber: SQLUSMALLINT,
	targetType: SQLSMALLINT,
	targetValuePtr: SQLPOINTER,
	bufferLength: SQLLEN,
	strLen_or_Ind: *const SQLLEN) -> SQLRETURN {

	return SQL_SUCCESS;
}

//TODO: SQLBindParameter
//TODO: SQLBrowseConnectW
//TODO: SQLBulkOperations

#[no_mangle]
pub extern fn SQLCancel(
	statementHandle: SQLHSTMT) -> SQLRETURN {

	return SQL_SUCCESS;
}

//SQLCloseCursor
//SQLColAttributeW
//SQLColumnPrivilegesW
//SQLColumnsW

#[no_mangle]
pub extern fn SQLConnectW(
	connectionHandle: SQLHDBC,
	serverName: *const SQLCHAR,
	nameLength1: SQLSMALLINT,
	userName: *const SQLCHAR,
	nameLength2: SQLSMALLINT,
	authentication: *const SQLCHAR,
	nameLength3: SQLSMALLINT) -> SQLRETURN {

	return SQL_SUCCESS;
}

//SQLCopyDesc
//SQLDebug
//SQLDescribeColW
//SQLDescribeParam
//SQLDisconnect

#[no_mangle]
pub extern fn SQLDriverConnectW(
	connectionHandle: SQLHDBC,
	windowHandle: SQLHWND,
	inConnectionString: *const SQLCHAR,
	stringLength1: SQLSMALLINT,
	outConnectionString: *mut SQLCHAR,
	bufferLength: SQLSMALLINT,
	stringLength2Ptr: *mut SQLSMALLINT,
	driverCompletion: SQLUSMALLINT) -> SQLRETURN {

	unsafe {
		std::ptr::copy_nonoverlapping(inConnectionString, outConnectionString, stringLength1 as usize);
		std::ptr::write(stringLength2Ptr, stringLength1);
	}

	return SQL_SUCCESS;
}

//SQLEndTran
//SQLExecDirectW
//SQLExecute
//SQLExtendedFetch
//SQLFetch
//SQLFetchScroll
//SQLForeignKeysW
//SQLFreeHandle
//SQLFreeStmt
//SQLGetConnectAttrW
//SQLGetConnectOptionW
//SQLGetCursorNameW
//SQLGetData
//SQLGetDescFieldW
//SQLGetDescRecW
//SQLGetDiagFieldW
//SQLGetDiagRecW
//SQLGetEnvAttr
//SQLGetFunctions
//SQLGetInfoW
//SQLGetStmtAttrW
//SQLGetTypeInfoW
//SQLMoreResults
//SQLNativeSqlW
//SQLNumParams
//SQLNumResultCols
//SQLParamData
//SQLParamOptions
//SQLPrepareW
//SQLPrimaryKeysW
//SQLProcedureColumnsW
//SQLProceduresW
//SQLPutData
//SQLRowCount
//SQLSetConnectAttrW
//SQLSetConnectOptionW
//SQLSetCursorNameW
//SQLSetDescFieldW
//SQLSetDescRec
//SQLSetEnvAttr
//SQLSetPos
//SQLSetScrollOptions
//SQLSetStmtAttrW
//SQLSpecialColumnsW
//SQLStatisticsW
//SQLTablePrivilegesW
//SQLTablesW

//TestDlgProc
//WizDSNDlgProc
//WizDatabaseDlgProc
//WizIntSecurityDlgProc
//WizLanguageDlgProc

//BCP_batch
//BCP_bind
//BCP_colfmt
//BCP_collen
//BCP_colptr
//BCP_columns
//BCP_control
//BCP_done
//BCP_exec
//BCP_getcolfmt
//BCP_init
//BCP_moretext
//BCP_readfmt
//BCP_sendrow
//BCP_setcolfmt
//BCP_writefmt
//ConfigDSNW
//ConfigDriverW
//ConnectDlgProc
//FinishDlgProc
//LibMain