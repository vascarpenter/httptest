DROP TABLE "BTUSERS";
CREATE TABLE "ADMIN"."BTUSERS" 
   (	"ID" NUMBER GENERATED BY DEFAULT ON NULL AS IDENTITY MINVALUE 1 MAXVALUE 9999999999999999999999999999 INCREMENT BY 1 START WITH 1 CACHE 20 NOORDER  NOCYCLE  NOKEEP  NOSCALE  NOT NULL ENABLE, 
	"USERID" NVARCHAR2(20) COLLATE "USING_NLS_COMP" NOT NULL ENABLE, 
	"USERPASS" NVARCHAR2(80) COLLATE "USING_NLS_COMP" NOT NULL ENABLE, 
	 PRIMARY KEY ("ID")
  USING INDEX PCTFREE 10 INITRANS 2 MAXTRANS 255 NOLOGGING 
  STORAGE(INITIAL 65536 NEXT 1048576 MINEXTENTS 1 MAXEXTENTS 2147483645
  PCTINCREASE 0 FREELISTS 1 FREELIST GROUPS 1
  BUFFER_POOL DEFAULT FLASH_CACHE DEFAULT CELL_FLASH_CACHE DEFAULT)
  TABLESPACE "DATA"  ENABLE
   )  DEFAULT COLLATION "USING_NLS_COMP" SEGMENT CREATION IMMEDIATE 
  PCTFREE 10 PCTUSED 40 INITRANS 1 MAXTRANS 255 
 NOCOMPRESS LOGGING
  STORAGE(INITIAL 65536 NEXT 1048576 MINEXTENTS 1 MAXEXTENTS 2147483645
  PCTINCREASE 0 FREELISTS 1 FREELIST GROUPS 1
  BUFFER_POOL DEFAULT FLASH_CACHE DEFAULT CELL_FLASH_CACHE DEFAULT)
  TABLESPACE "DATA";

SET DEFINE OFF;
Insert Into BTUSERS ("ID","USERID","USERPASS") VALUES (1,'test','$2a$10$pF83wGcVerai4s3ZpRTDYegJ2XSaMUyWtGM7Sk1CadlUZfWlLkof2');


DROP TABLE "BTDATA";
CREATE TABLE "ADMIN"."BTDATA" 
   (	"ID" NUMBER NOT NULL ENABLE, 
	"DATE" DATE NOT NULL ENABLE, 
	"TEMP" NUMBER NOT NULL ENABLE, 
	"MEMO" VARCHAR2(100 CHAR) COLLATE "USING_NLS_COMP"
   )  DEFAULT COLLATION "USING_NLS_COMP" SEGMENT CREATION IMMEDIATE 
  PCTFREE 10 PCTUSED 40 INITRANS 10 MAXTRANS 255 
 NOCOMPRESS LOGGING
  STORAGE(INITIAL 65536 NEXT 1048576 MINEXTENTS 1 MAXEXTENTS 2147483645
  PCTINCREASE 0 FREELISTS 1 FREELIST GROUPS 1
  BUFFER_POOL DEFAULT FLASH_CACHE DEFAULT CELL_FLASH_CACHE DEFAULT)
  TABLESPACE "DATA";

SET DEFINE OFF;
Insert Into BTDATA ("ID","DATE","TEMP","MEMO") VALUES (1,TO_DATE('21-10-18','RR-MM-DD'),35.5,'oge');
Insert Into BTDATA ("ID","DATE","TEMP","MEMO") VALUES (1,TO_DATE('21-10-18','RR-MM-DD'),34.7,'');
Insert Into BTDATA ("ID","DATE","TEMP","MEMO") VALUES (1,TO_DATE('21-10-18','RR-MM-DD'),36,'');
Insert Into BTDATA ("ID","DATE","TEMP","MEMO") VALUES (1,TO_DATE('21-10-18','RR-MM-DD'),36,'');
Insert Into BTDATA ("ID","DATE","TEMP","MEMO") VALUES (1,TO_DATE('21-10-18','RR-MM-DD'),37,'');


