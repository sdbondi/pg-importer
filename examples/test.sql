--
-- PostgreSQL database dump
--

-- Dumped from database version 9.6.10
-- Dumped by pg_dump version 11.1

-- Started on 2019-03-15 13:31:23 SAST

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 2 (class 3079 OID 253688)
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "pgcrypto" WITH SCHEMA "public";


--
-- TOC entry 2741 (class 0 OID 0)
-- Dependencies: 2
-- Name: EXTENSION "pgcrypto"; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION "pgcrypto" IS 'cryptographic functions';


--
-- TOC entry 268 (class 1255 OID 253725)
-- Name: diesel_manage_updated_at("regclass"); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION "public"."diesel_manage_updated_at"("_tbl" "regclass") RETURNS "void"
    LANGUAGE "plpgsql"
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


--
-- TOC entry 269 (class 1255 OID 253726)
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION "public"."diesel_set_updated_at"() RETURNS "trigger"
    LANGUAGE "plpgsql"
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;



SET default_tablespace = '';

SET default_with_oids = false;

--
-- TOC entry 186 (class 1259 OID 253682)
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE "public"."__diesel_schema_migrations" (
    "version" character varying(50) NOT NULL,
    "run_on" timestamp without time zone DEFAULT "now"() NOT NULL
);


--
-- TOC entry 187 (class 1259 OID 253727)
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE "public"."users" (
    "id" "uuid" DEFAULT "public"."gen_random_uuid"() NOT NULL,
    "first_name" "text",
    "last_name" "text",
    "email" "text",
    "phone" "text",
    "profile_pic_url" "text",
    "thumb_profile_pic_url" "text",
    "cover_photo_url" "text",
    "hashed_pw" "text" NOT NULL,
    "password_modified_at" timestamp without time zone DEFAULT "now"() NOT NULL,
    "created_at" timestamp without time zone DEFAULT "now"() NOT NULL,
    "last_used" timestamp without time zone,
    "updated_at" timestamp without time zone DEFAULT "now"() NOT NULL
);


--
-- TOC entry 2701 (class 0 OID 253682)
-- Dependencies: 186
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000000', '2019-03-11 06:26:52.968938');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000002', '2019-03-11 06:26:52.980115');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000003', '2019-03-11 06:26:52.994862');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000004', '2019-03-11 06:26:53.009115');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000005', '2019-03-11 06:26:53.02189');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000006', '2019-03-11 06:26:53.031543');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000007', '2019-03-11 06:26:53.04486');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000009', '2019-03-11 06:26:53.057354');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000010', '2019-03-11 06:26:53.070063');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000012', '2019-03-11 06:26:53.080966');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000013', '2019-03-11 06:26:53.099036');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000014', '2019-03-11 06:26:53.110469');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000016', '2019-03-11 06:26:53.119781');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000020', '2019-03-11 06:26:53.132772');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000021', '2019-03-11 06:26:53.146515');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000022', '2019-03-11 06:26:53.161339');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000023', '2019-03-11 06:26:53.174387');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000024', '2019-03-11 06:26:53.186119');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000025', '2019-03-11 06:26:53.196779');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000027', '2019-03-11 06:26:53.212234');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000028', '2019-03-11 06:26:53.225529');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000029', '2019-03-11 06:26:53.24254');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000031', '2019-03-11 06:26:53.264387');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000032', '2019-03-11 06:26:53.276848');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000033', '2019-03-11 06:26:53.289132');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000034', '2019-03-11 06:26:53.293577');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000035', '2019-03-11 06:26:53.305037');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000036', '2019-03-11 06:26:53.312978');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000037', '2019-03-11 06:26:53.318015');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000039', '2019-03-11 06:26:53.325247');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000040', '2019-03-11 06:26:53.34085');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000041', '2019-03-11 06:26:53.34921');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000043', '2019-03-11 06:26:53.354634');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000044', '2019-03-11 06:26:53.368988');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000045', '2019-03-11 06:26:53.381914');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000046', '2019-03-11 06:26:53.406097');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000047', '2019-03-11 06:26:53.414929');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000048', '2019-03-11 06:26:53.419515');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000049', '2019-03-11 06:26:53.424608');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000050', '2019-03-11 06:26:53.436309');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000051', '2019-03-11 06:26:53.441023');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000052', '2019-03-11 06:26:53.446712');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000053', '2019-03-11 06:26:53.452292');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000054', '2019-03-11 06:26:53.457037');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000055', '2019-03-11 06:26:53.461917');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000056', '2019-03-11 06:26:53.504675');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000057', '2019-03-11 06:26:53.509403');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000058', '2019-03-11 06:26:53.514024');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000059', '2019-03-11 06:26:53.51791');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('00000000000060', '2019-03-11 06:26:53.523712');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20181221171123', '2019-03-11 06:26:53.53396');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20181231062704', '2019-03-11 06:26:53.558768');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20181231070926', '2019-03-11 06:26:53.562819');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20181231073456', '2019-03-11 06:26:53.566615');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20181231091032', '2019-03-11 06:26:53.577768');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190103085424', '2019-03-11 06:26:53.582262');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190103133201', '2019-03-11 06:26:53.5866');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190104115139', '2019-03-11 06:26:53.590223');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190109095000', '2019-03-11 06:26:53.593749');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190109095151', '2019-03-11 06:26:53.604915');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190114161736', '2019-03-11 06:26:53.61964');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190117145558', '2019-03-11 06:26:53.624678');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190118094738', '2019-03-11 06:26:53.629265');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190121140415', '2019-03-11 06:26:53.63454');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190122212554', '2019-03-11 06:26:53.639612');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190125172528', '2019-03-11 06:26:53.64448');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190128081807', '2019-03-11 06:26:53.650052');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190128112408', '2019-03-11 06:26:53.665792');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190131112715', '2019-03-11 06:26:53.671443');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190201162057', '2019-03-11 06:26:53.684014');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190202042857', '2019-03-11 06:26:53.709993');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190206111804', '2019-03-11 06:26:53.720179');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190206115503', '2019-03-11 06:26:53.725242');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190206204219', '2019-03-11 06:26:53.73054');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190207083449', '2019-03-11 06:26:53.734842');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190208114032', '2019-03-11 06:26:53.740312');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190209073214', '2019-03-11 06:26:53.744857');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190211080428', '2019-03-11 06:26:53.750521');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190211090307', '2019-03-11 06:26:53.755488');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190211104214', '2019-03-11 06:26:53.775315');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190211125037', '2019-03-11 06:26:53.779549');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190213175234', '2019-03-11 06:26:53.789589');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190214142752', '2019-03-11 06:26:53.793039');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190218153958', '2019-03-11 06:26:53.805217');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190220123908', '2019-03-11 06:26:53.817042');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190222143417', '2019-03-11 06:26:53.827725');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190226162429', '2019-03-11 06:26:53.832242');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190228145342', '2019-03-11 06:26:53.836491');
INSERT INTO "public"."__diesel_schema_migrations" VALUES ('20190304185928', '2019-03-11 06:26:53.841134');

--
-- TOC entry 2378 (class 2606 OID 253687)
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."__diesel_schema_migrations"
    ADD CONSTRAINT "__diesel_schema_migrations_pkey" PRIMARY KEY ("version");


--
-- TOC entry 2384 (class 2606 OID 253741)
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."users"
    ADD CONSTRAINT "users_email_key" UNIQUE ("email");


--
-- TOC entry 2386 (class 2606 OID 253739)
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--


ALTER TABLE ONLY "public"."users"
    ADD CONSTRAINT "users_pkey" PRIMARY KEY ("id");
--
-- TOC entry 2379 (class 1259 OID 253743)
-- Name: index_users_email; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX "index_users_email" ON "public"."users" USING "btree" ("email")

--
-- TOC entry 2381 (class 1259 OID 253744)
-- Name: index_users_password_reset_token; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX "index_users_password_reset_token" ON "public"."users" USING "btree" ("password_reset_token");


--
-- TOC entry 2382 (class 1259 OID 253742)
-- Name: index_users_uuid; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX "index_users_uuid" ON "public"."users" USING "btree" ("id");


--
-- TOC entry 2527 (class 2606 OID 254280)
-- Name: users users_last_cart_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."users"
    ADD CONSTRAINT "users_last_cart_id_fkey" FOREIGN KEY ("last_cart_id") REFERENCES "public"."orders"("id");


--
-- TOC entry 2740 (class 0 OID 0)
-- Dependencies: 4
-- Name: SCHEMA "public"; Type: ACL; Schema: -; Owner: -
--

REVOKE ALL ON SCHEMA "public" FROM PUBLIC;


-- Completed on 2019-03-15 13:31:27 SAST

--
-- PostgreSQL database dump complete
--

