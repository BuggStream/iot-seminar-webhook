--
-- PostgreSQL database dump
--

-- Dumped from database version 14.13 (Ubuntu 14.13-0ubuntu0.22.04.1)
-- Dumped by pg_dump version 14.13 (Ubuntu 14.13-0ubuntu0.22.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: join; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."join" (
    id bigint NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    message json NOT NULL
);


ALTER TABLE public."join" OWNER TO postgres;

--
-- Name: join_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public."join" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.join_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: location; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.location (
    id bigint NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    message json NOT NULL
);


ALTER TABLE public.location OWNER TO postgres;

--
-- Name: location_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public.location ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.location_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: uplink; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.uplink (
    id bigint NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    message json NOT NULL
);


ALTER TABLE public.uplink OWNER TO postgres;

--
-- Name: uplink_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public.uplink ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.uplink_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: uplink id; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.uplink
    ADD CONSTRAINT id PRIMARY KEY (id);


--
-- Name: join join_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."join"
    ADD CONSTRAINT join_pkey PRIMARY KEY (id);


--
-- Name: location location_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.location
    ADD CONSTRAINT location_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

