CREATE TABLE "academic_study_pairs" (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    teacher_id UUID NOT NULL REFERENCES "users" (id), -- References the "users" table for the teacher
    learner_id UUID NOT NULL REFERENCES "users" (id), -- References the "users" table for the learner
    teacher_accepted BOOLEAN DEFAULT FALSE, -- Indicates if the teacher has accepted the study
    subject VARCHAR(100) NOT NULL, -- Subject of the study
    description TEXT, -- Description of the study (can be longer text)
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);